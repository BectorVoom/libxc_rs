//! MGGA_X_TM vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tm_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(1.0) / v_rho0;
            let t30 = v_sigma0 * t29;
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t33 = t30 * t31 / f64x8::splat(8.0);
            let t34 = (t33).simd_lt(f64x8::splat(1.0));
            let t35 = ((t34).select(t33, f64x8::splat(1.0)));
            let t36 = t35 * t35;
            let t37 = t36 * t35;
            let t39 = t36 + f64x8::splat(3.0) * t37;
            let t40 = f64x8::splat(1.0) + t37;
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t39 * t42;
            let t44 = f64x8::splat(M_CBRT6);
            let t45 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t46 = (simd::cbrt(t45));
            let t47 = t46 * t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t44 * t48;
            let t50 = v_rho0 * v_rho0;
            let t51 = (simd::cbrt(v_rho0));
            let t52 = t51 * t51;
            let t54 = f64x8::splat(1.0) / t52 / t50;
            let t55 = v_sigma0 * t54;
            let t56 = t49 * t55;
            let t58 = t44 * t44;
            let t60 = f64x8::splat(1.0) / t46 / t45;
            let t61 = t58 * t60;
            let t62 = v_sigma0 * v_sigma0;
            let t63 = t50 * t50;
            let t64 = t63 * v_rho0;
            let t66 = f64x8::splat(1.0) / t51 / t64;
            let t70 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t56 + f64x8::splat(0.002689949046226295) * t61 * t62 * t66;
            let t71 = (simd::pow(t70, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t75 = f64x8::splat(1.0) / t52 / v_rho0;
            let t76 = v_tau0 * t75;
            let t79 = f64x8::splat(0.256337604) * t58 * t47;
            let t85 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t56 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t76 + t79 + f64x8::splat(0.011867481666666667) * t55) * t44 * t48;
            let t86 = t71 * t71;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = f64x8::splat(1.0) / t71 + f64x8::splat(7.0) / f64x8::splat(9.0) * t85 * t87;
            let t92 = f64x8::splat(1.0) - t43;
            let t95 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t56) * t44;
            let t96 = t48 * v_sigma0;
            let t106 = (t76 - t55 / f64x8::splat(8.0)) * t44 * t48 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t56 / f64x8::splat(36.0);
            let t107 = t106 * t106;
            let t109 = t106 * t35;
            let t110 = f64x8::splat(1.0) - t35;
            let t113 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t95 * t96 * t54 + f64x8::splat(292.0) / f64x8::splat(405.0) * t107 - f64x8::splat(146.0) / f64x8::splat(135.0) * t109 * t110;
            let t114 = (simd::pow(t113, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t116 = t92 * t114 + t43 * t90;
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t116));
            let t121 = (v_rho1).simd_le(dens_threshold);
            let t122 = -t17;
            let t124 = ((t15).select(t12, (t11).select(t16, t122 * t8)));
            let t125 = f64x8::splat(1.0) + t124;
            let t126 = (t125).simd_le(zeta_threshold);
            let t127 = (simd::cbrt(t125));
            let t129 = ((t126).select(t23, t127 * t125));
            let t130 = t129 * t27;
            let t131 = f64x8::splat(1.0) / v_rho1;
            let t132 = v_sigma2 * t131;
            let t133 = f64x8::splat(1.0) / v_tau1;
            let t135 = t132 * t133 / f64x8::splat(8.0);
            let t136 = (t135).simd_lt(f64x8::splat(1.0));
            let t137 = ((t136).select(t135, f64x8::splat(1.0)));
            let t138 = t137 * t137;
            let t139 = t138 * t137;
            let t141 = t138 + f64x8::splat(3.0) * t139;
            let t142 = f64x8::splat(1.0) + t139;
            let t143 = t142 * t142;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t141 * t144;
            let t146 = v_rho1 * v_rho1;
            let t147 = (simd::cbrt(v_rho1));
            let t148 = t147 * t147;
            let t150 = f64x8::splat(1.0) / t148 / t146;
            let t151 = v_sigma2 * t150;
            let t152 = t49 * t151;
            let t154 = v_sigma2 * v_sigma2;
            let t155 = t146 * t146;
            let t156 = t155 * v_rho1;
            let t158 = f64x8::splat(1.0) / t147 / t156;
            let t162 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t152 + f64x8::splat(0.002689949046226295) * t61 * t154 * t158;
            let t163 = (simd::pow(t162, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t167 = f64x8::splat(1.0) / t148 / v_rho1;
            let t168 = v_tau1 * t167;
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t152 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t168 + t79 + f64x8::splat(0.011867481666666667) * t151) * t44 * t48;
            let t176 = t163 * t163;
            let t177 = f64x8::splat(1.0) / t176;
            let t180 = f64x8::splat(1.0) / t163 + f64x8::splat(7.0) / f64x8::splat(9.0) * t175 * t177;
            let t182 = f64x8::splat(1.0) - t145;
            let t185 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t152) * t44;
            let t186 = t48 * v_sigma2;
            let t196 = (t168 - t151 / f64x8::splat(8.0)) * t44 * t48 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t152 / f64x8::splat(36.0);
            let t197 = t196 * t196;
            let t199 = t196 * t137;
            let t200 = f64x8::splat(1.0) - t137;
            let t203 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t185 * t186 * t150 + f64x8::splat(292.0) / f64x8::splat(405.0) * t197 - f64x8::splat(146.0) / f64x8::splat(135.0) * t199 * t200;
            let t204 = (simd::pow(t203, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t206 = t145 * t180 + t182 * t204;
            let t210 = ((t121).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t130 * t206));
            let tzk0 = t120 + t210;
            acc_zk = tzk0;
            let t211 = t7 * t7;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = t17 * t212;
            let t215 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t213)));
            let t218 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t215));
            let t219 = t218 * t27;
            let t223 = t27 * t27;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t26 * t224;
            let t228 = t6 * t225 * t116 / f64x8::splat(8.0);
            let t229 = f64x8::splat(1.0) / t50;
            let t230 = v_sigma0 * t229;
            let t233 = ((t34).select(-t230 * t31 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t234 = t35 * t233;
            let t236 = t36 * t233;
            let t238 = f64x8::splat(2.0) * t234 + f64x8::splat(9.0) * t236;
            let t239 = t238 * t42;
            let t242 = f64x8::splat(1.0) / t41 / t40;
            let t243 = t39 * t242;
            let t244 = t90 * t36;
            let t245 = t244 * t233;
            let t249 = f64x8::splat(1.0) / t71 / t70;
            let t250 = t50 * v_rho0;
            let t252 = f64x8::splat(1.0) / t52 / t250;
            let t253 = v_sigma0 * t252;
            let t254 = t49 * t253;
            let t256 = t63 * t50;
            let t258 = f64x8::splat(1.0) / t51 / t256;
            let t260 = t61 * t62 * t258;
            let t262 = -f64x8::splat(0.40121303703703703) * t254 - f64x8::splat(0.014346394913206906) * t260;
            let t266 = v_tau0 * t54;
            let t273 = -f64x8::splat(0.17051554074074074) * t254 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t266 - f64x8::splat(0.031646617777777775) * t253) * t44 * t48;
            let t277 = f64x8::splat(1.0) / t86 / t70;
            let t278 = t85 * t277;
            let t281 = -t249 * t262 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t273 * t87 - f64x8::splat(14.0) / f64x8::splat(45.0) * t278 * t262;
            let t285 = f64x8::splat(6.0) * t243 * t236 - t239;
            let t287 = t114 * t114;
            let t288 = t287 * t287;
            let t289 = t288 * t288;
            let t290 = t289 * t114;
            let t291 = f64x8::splat(1.0) / t290;
            let t292 = t92 * t291;
            let t304 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t266 + t253 / f64x8::splat(3.0)) * t44 * t48 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t254;
            let t307 = t304 * t35;
            let t310 = t106 * t233;
            let t315 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t260 - f64x8::splat(10.0) / f64x8::splat(9.0) * t95 * t96 * t252 + f64x8::splat(584.0) / f64x8::splat(405.0) * t106 * t304 - f64x8::splat(146.0) / f64x8::splat(135.0) * t307 * t110 - f64x8::splat(146.0) / f64x8::splat(135.0) * t310 * t110 + f64x8::splat(146.0) / f64x8::splat(135.0) * t109 * t233;
            let t318 = t239 * t90 - f64x8::splat(6.0) * t243 * t245 + t43 * t281 + t285 * t114 + t292 * t315 / f64x8::splat(10.0);
            let t323 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t219 * t116 - t228 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t318));
            let t324 = t122 * t212;
            let t326 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t324)));
            let t329 = ((t126).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t127 * t326));
            let t330 = t329 * t27;
            let t334 = t129 * t224;
            let t337 = t6 * t334 * t206 / f64x8::splat(8.0);
            let t339 = ((t121).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t330 * t206 - t337));
            let tvrho0 = t120 + t210 + t7 * (t323 + t339);
            acc_vrho_0 = tvrho0;
            let t343 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t213)));
            let t346 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t343));
            let t347 = t346 * t27;
            let t352 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t347 * t116 - t228));
            let t354 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t324)));
            let t357 = ((t126).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t127 * t354));
            let t358 = t357 * t27;
            let t362 = f64x8::splat(1.0) / t146;
            let t363 = v_sigma2 * t362;
            let t366 = ((t136).select(-t363 * t133 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t367 = t137 * t366;
            let t369 = t138 * t366;
            let t371 = f64x8::splat(2.0) * t367 + f64x8::splat(9.0) * t369;
            let t372 = t371 * t144;
            let t375 = f64x8::splat(1.0) / t143 / t142;
            let t376 = t141 * t375;
            let t377 = t180 * t138;
            let t378 = t377 * t366;
            let t382 = f64x8::splat(1.0) / t163 / t162;
            let t383 = t146 * v_rho1;
            let t385 = f64x8::splat(1.0) / t148 / t383;
            let t386 = v_sigma2 * t385;
            let t387 = t49 * t386;
            let t389 = t155 * t146;
            let t391 = f64x8::splat(1.0) / t147 / t389;
            let t393 = t61 * t154 * t391;
            let t395 = -f64x8::splat(0.40121303703703703) * t387 - f64x8::splat(0.014346394913206906) * t393;
            let t399 = v_tau1 * t150;
            let t406 = -f64x8::splat(0.17051554074074074) * t387 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t399 - f64x8::splat(0.031646617777777775) * t386) * t44 * t48;
            let t410 = f64x8::splat(1.0) / t176 / t162;
            let t411 = t175 * t410;
            let t414 = -t382 * t395 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t406 * t177 - f64x8::splat(14.0) / f64x8::splat(45.0) * t411 * t395;
            let t418 = f64x8::splat(6.0) * t376 * t369 - t372;
            let t420 = t204 * t204;
            let t421 = t420 * t420;
            let t422 = t421 * t421;
            let t423 = t422 * t204;
            let t424 = f64x8::splat(1.0) / t423;
            let t425 = t182 * t424;
            let t437 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t399 + t386 / f64x8::splat(3.0)) * t44 * t48 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t387;
            let t440 = t437 * t137;
            let t448 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t393 - f64x8::splat(10.0) / f64x8::splat(9.0) * t185 * t186 * t385 + f64x8::splat(584.0) / f64x8::splat(405.0) * t196 * t437 - f64x8::splat(146.0) / f64x8::splat(135.0) * t440 * t200 - f64x8::splat(146.0) / f64x8::splat(135.0) * t196 * t366 * t200 + f64x8::splat(146.0) / f64x8::splat(135.0) * t199 * t366;
            let t451 = t372 * t180 - f64x8::splat(6.0) * t376 * t378 + t145 * t414 + t418 * t204 + t425 * t448 / f64x8::splat(10.0);
            let t456 = ((t121).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t358 * t206 - t337 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t130 * t451));
            let tvrho1 = t120 + t210 + t7 * (t352 + t456);
            acc_vrho_1 = tvrho1;
            let t461 = ((t34).select(t29 * t31 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t462 = t35 * t461;
            let t464 = t36 * t461;
            let t466 = f64x8::splat(2.0) * t462 + f64x8::splat(9.0) * t464;
            let t467 = t466 * t42;
            let t469 = t244 * t461;
            let t472 = t49 * t54;
            let t475 = t61 * v_sigma0 * t66;
            let t477 = f64x8::splat(0.1504548888888889) * t472 + f64x8::splat(0.00537989809245259) * t475;
            let t485 = -t249 * t477 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t49 * t54 * t87 - f64x8::splat(14.0) / f64x8::splat(45.0) * t278 * t477;
            let t489 = f64x8::splat(6.0) * t243 * t464 - t467;
            let t492 = t48 * t54;
            let t495 = t106 * t44;
            let t496 = t495 * t492;
            let t498 = t54 * t35;
            let t500 = t49 * t498 * t110;
            let t502 = t106 * t461;
            let t507 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t475 + f64x8::splat(5.0) / f64x8::splat(12.0) * t95 * t492 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t496 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t500 - f64x8::splat(146.0) / f64x8::splat(135.0) * t502 * t110 + f64x8::splat(146.0) / f64x8::splat(135.0) * t109 * t461;
            let t510 = t467 * t90 - f64x8::splat(6.0) * t243 * t469 + t43 * t485 + t489 * t114 + t292 * t507 / f64x8::splat(10.0);
            let t514 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t510));
            let tvsigma0 = t7 * t514;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t517 = ((t136).select(t131 * t133 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t518 = t137 * t517;
            let t520 = t138 * t517;
            let t522 = f64x8::splat(2.0) * t518 + f64x8::splat(9.0) * t520;
            let t523 = t522 * t144;
            let t525 = t377 * t517;
            let t528 = t49 * t150;
            let t531 = t61 * v_sigma2 * t158;
            let t533 = f64x8::splat(0.1504548888888889) * t528 + f64x8::splat(0.00537989809245259) * t531;
            let t541 = -t382 * t533 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t49 * t150 * t177 - f64x8::splat(14.0) / f64x8::splat(45.0) * t411 * t533;
            let t545 = f64x8::splat(6.0) * t376 * t520 - t523;
            let t548 = t48 * t150;
            let t551 = t196 * t44;
            let t552 = t551 * t548;
            let t554 = t150 * t137;
            let t556 = t49 * t554 * t200;
            let t558 = t196 * t517;
            let t563 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t531 + f64x8::splat(5.0) / f64x8::splat(12.0) * t185 * t548 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t552 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t556 - f64x8::splat(146.0) / f64x8::splat(135.0) * t558 * t200 + f64x8::splat(146.0) / f64x8::splat(135.0) * t199 * t517;
            let t566 = t523 * t180 - f64x8::splat(6.0) * t376 * t525 + t145 * t541 + t545 * t204 + t425 * t563 / f64x8::splat(10.0);
            let t570 = ((t121).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t130 * t566));
            let tvsigma2 = t7 * t570;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t571 = v_tau0 * v_tau0;
            let t572 = f64x8::splat(1.0) / t571;
            let t575 = ((t34).select(-t30 * t572 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t576 = t35 * t575;
            let t578 = t36 * t575;
            let t580 = f64x8::splat(2.0) * t576 + f64x8::splat(9.0) * t578;
            let t581 = t580 * t42;
            let t583 = t244 * t575;
            let t586 = t43 * t75;
            let t587 = t49 * t87;
            let t592 = f64x8::splat(6.0) * t243 * t578 - t581;
            let t597 = t75 * t44;
            let t598 = t48 * t35;
            let t602 = t106 * t575;
            let t607 = f64x8::splat(146.0) / f64x8::splat(405.0) * t106 * t75 * t49 - f64x8::splat(73.0) / f64x8::splat(270.0) * t597 * t598 * t110 - f64x8::splat(146.0) / f64x8::splat(135.0) * t602 * t110 + f64x8::splat(146.0) / f64x8::splat(135.0) * t109 * t575;
            let t610 = t581 * t90 - f64x8::splat(6.0) * t243 * t583 - f64x8::splat(0.06288822469135802) * t586 * t587 + t592 * t114 + t292 * t607 / f64x8::splat(10.0);
            let t614 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t610));
            let tvtau0 = t7 * t614;
            acc_vtau_0 = tvtau0;
            let t615 = v_tau1 * v_tau1;
            let t616 = f64x8::splat(1.0) / t615;
            let t619 = ((t136).select(-t132 * t616 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t620 = t137 * t619;
            let t622 = t138 * t619;
            let t624 = f64x8::splat(2.0) * t620 + f64x8::splat(9.0) * t622;
            let t625 = t624 * t144;
            let t627 = t377 * t619;
            let t630 = t145 * t167;
            let t631 = t49 * t177;
            let t636 = f64x8::splat(6.0) * t376 * t622 - t625;
            let t641 = t167 * t44;
            let t642 = t48 * t137;
            let t646 = t196 * t619;
            let t651 = f64x8::splat(146.0) / f64x8::splat(405.0) * t196 * t167 * t49 - f64x8::splat(73.0) / f64x8::splat(270.0) * t641 * t642 * t200 - f64x8::splat(146.0) / f64x8::splat(135.0) * t646 * t200 + f64x8::splat(146.0) / f64x8::splat(135.0) * t199 * t619;
            let t654 = t625 * t180 - f64x8::splat(6.0) * t376 * t627 - f64x8::splat(0.06288822469135802) * t630 * t631 + t636 * t204 + t425 * t651 / f64x8::splat(10.0);
            let t658 = ((t121).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t130 * t654));
            let tvtau1 = t7 * t658;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
