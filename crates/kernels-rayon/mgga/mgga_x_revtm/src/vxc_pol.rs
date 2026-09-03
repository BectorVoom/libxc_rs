//! MGGA_X_REVTM vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`
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
pub fn mgga_x_revtm_vxc_pol(
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
            let t101 = t76 - t55 / f64x8::splat(8.0);
            let t102 = t101 * t44;
            let t105 = f64x8::splat(5.0) / f64x8::splat(9.0) * t102 * t48 - f64x8::splat(1.0);
            let t106 = t48 * t105;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t102 * t106;
            let t110 = ((t109).sqrt());
            let t111 = f64x8::splat(1.0) / t110;
            let t115 = f64x8::splat(9.0) / f64x8::splat(20.0) * t105 * t111 + t56 / f64x8::splat(36.0);
            let t116 = t115 * t115;
            let t118 = t115 * t35;
            let t119 = f64x8::splat(1.0) - t35;
            let t122 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t95 * t96 * t54 + f64x8::splat(292.0) / f64x8::splat(405.0) * t116 - f64x8::splat(146.0) / f64x8::splat(135.0) * t118 * t119;
            let t123 = (simd::pow(t122, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t125 = t92 * t123 + t43 * t90;
            let t129 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t125));
            let t130 = (v_rho1).simd_le(dens_threshold);
            let t131 = -t17;
            let t133 = ((t15).select(t12, (t11).select(t16, t131 * t8)));
            let t134 = f64x8::splat(1.0) + t133;
            let t135 = (t134).simd_le(zeta_threshold);
            let t136 = (simd::cbrt(t134));
            let t138 = ((t135).select(t23, t136 * t134));
            let t139 = t138 * t27;
            let t140 = f64x8::splat(1.0) / v_rho1;
            let t141 = v_sigma2 * t140;
            let t142 = f64x8::splat(1.0) / v_tau1;
            let t144 = t141 * t142 / f64x8::splat(8.0);
            let t145 = (t144).simd_lt(f64x8::splat(1.0));
            let t146 = ((t145).select(t144, f64x8::splat(1.0)));
            let t147 = t146 * t146;
            let t148 = t147 * t146;
            let t150 = t147 + f64x8::splat(3.0) * t148;
            let t151 = f64x8::splat(1.0) + t148;
            let t152 = t151 * t151;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t153;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / t155;
            let t160 = v_sigma2 * t159;
            let t161 = t49 * t160;
            let t163 = v_sigma2 * v_sigma2;
            let t164 = t155 * t155;
            let t165 = t164 * v_rho1;
            let t167 = f64x8::splat(1.0) / t156 / t165;
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t161 + f64x8::splat(0.002689949046226295) * t61 * t163 * t167;
            let t172 = (simd::pow(t171, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t176 = f64x8::splat(1.0) / t157 / v_rho1;
            let t177 = v_tau1 * t176;
            let t184 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t161 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t177 + t79 + f64x8::splat(0.011867481666666667) * t160) * t44 * t48;
            let t185 = t172 * t172;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = f64x8::splat(1.0) / t172 + f64x8::splat(7.0) / f64x8::splat(9.0) * t184 * t186;
            let t191 = f64x8::splat(1.0) - t154;
            let t194 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t161) * t44;
            let t195 = t48 * v_sigma2;
            let t200 = t177 - t160 / f64x8::splat(8.0);
            let t201 = t200 * t44;
            let t204 = f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t48 - f64x8::splat(1.0);
            let t205 = t48 * t204;
            let t208 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t201 * t205;
            let t209 = ((t208).sqrt());
            let t210 = f64x8::splat(1.0) / t209;
            let t214 = f64x8::splat(9.0) / f64x8::splat(20.0) * t204 * t210 + t161 / f64x8::splat(36.0);
            let t215 = t214 * t214;
            let t217 = t214 * t146;
            let t218 = f64x8::splat(1.0) - t146;
            let t221 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t194 * t195 * t159 + f64x8::splat(292.0) / f64x8::splat(405.0) * t215 - f64x8::splat(146.0) / f64x8::splat(135.0) * t217 * t218;
            let t222 = (simd::pow(t221, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t224 = t154 * t189 + t191 * t222;
            let t228 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t139 * t224));
            let tzk0 = t129 + t228;
            acc_zk = tzk0;
            let t229 = t7 * t7;
            let t230 = f64x8::splat(1.0) / t229;
            let t231 = t17 * t230;
            let t233 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t231)));
            let t236 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t233));
            let t237 = t236 * t27;
            let t241 = t27 * t27;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t26 * t242;
            let t246 = t6 * t243 * t125 / f64x8::splat(8.0);
            let t247 = f64x8::splat(1.0) / t50;
            let t248 = v_sigma0 * t247;
            let t251 = ((t34).select(-t248 * t31 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t252 = t35 * t251;
            let t254 = t36 * t251;
            let t256 = f64x8::splat(2.0) * t252 + f64x8::splat(9.0) * t254;
            let t257 = t256 * t42;
            let t260 = f64x8::splat(1.0) / t41 / t40;
            let t261 = t39 * t260;
            let t262 = t90 * t36;
            let t263 = t262 * t251;
            let t267 = f64x8::splat(1.0) / t71 / t70;
            let t268 = t50 * v_rho0;
            let t270 = f64x8::splat(1.0) / t52 / t268;
            let t271 = v_sigma0 * t270;
            let t272 = t49 * t271;
            let t274 = t63 * t50;
            let t276 = f64x8::splat(1.0) / t51 / t274;
            let t278 = t61 * t62 * t276;
            let t280 = -f64x8::splat(0.40121303703703703) * t272 - f64x8::splat(0.014346394913206906) * t278;
            let t284 = v_tau0 * t54;
            let t291 = -f64x8::splat(0.17051554074074074) * t272 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t284 - f64x8::splat(0.031646617777777775) * t271) * t44 * t48;
            let t295 = f64x8::splat(1.0) / t86 / t70;
            let t296 = t85 * t295;
            let t299 = -t267 * t280 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t291 * t87 - f64x8::splat(14.0) / f64x8::splat(45.0) * t296 * t280;
            let t303 = f64x8::splat(6.0) * t261 * t254 - t257;
            let t305 = t123 * t123;
            let t306 = t305 * t305;
            let t307 = t306 * t306;
            let t308 = t307 * t123;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t92 * t309;
            let t317 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t284 + t271 / f64x8::splat(3.0);
            let t318 = t317 * t44;
            let t319 = t48 * t111;
            let t323 = f64x8::splat(1.0) / t110 / t109;
            let t324 = t105 * t323;
            let t327 = t101 * t58;
            let t328 = t60 * t317;
            let t331 = f64x8::splat(0.2222222222222222) * t318 * t106 + f64x8::splat(0.12345679012345678) * t327 * t328;
            let t335 = t318 * t319 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t324 * t331 - f64x8::splat(2.0) / f64x8::splat(27.0) * t272;
            let t338 = t335 * t35;
            let t341 = t115 * t251;
            let t346 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t278 - f64x8::splat(10.0) / f64x8::splat(9.0) * t95 * t96 * t270 + f64x8::splat(584.0) / f64x8::splat(405.0) * t115 * t335 - f64x8::splat(146.0) / f64x8::splat(135.0) * t338 * t119 - f64x8::splat(146.0) / f64x8::splat(135.0) * t341 * t119 + f64x8::splat(146.0) / f64x8::splat(135.0) * t118 * t251;
            let t349 = t257 * t90 - f64x8::splat(6.0) * t261 * t263 + t43 * t299 + t303 * t123 + t310 * t346 / f64x8::splat(10.0);
            let t354 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t237 * t125 - t246 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t349));
            let t355 = t131 * t230;
            let t357 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t355)));
            let t360 = ((t135).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t136 * t357));
            let t361 = t360 * t27;
            let t365 = t138 * t242;
            let t368 = t6 * t365 * t224 / f64x8::splat(8.0);
            let t370 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t361 * t224 - t368));
            let tvrho0 = t129 + t228 + t7 * (t354 + t370);
            acc_vrho_0 = tvrho0;
            let t374 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t231)));
            let t377 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t374));
            let t378 = t377 * t27;
            let t383 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t378 * t125 - t246));
            let t385 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t355)));
            let t388 = ((t135).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t136 * t385));
            let t389 = t388 * t27;
            let t393 = f64x8::splat(1.0) / t155;
            let t394 = v_sigma2 * t393;
            let t397 = ((t145).select(-t394 * t142 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t398 = t146 * t397;
            let t400 = t147 * t397;
            let t402 = f64x8::splat(2.0) * t398 + f64x8::splat(9.0) * t400;
            let t403 = t402 * t153;
            let t406 = f64x8::splat(1.0) / t152 / t151;
            let t407 = t150 * t406;
            let t408 = t189 * t147;
            let t409 = t408 * t397;
            let t413 = f64x8::splat(1.0) / t172 / t171;
            let t414 = t155 * v_rho1;
            let t416 = f64x8::splat(1.0) / t157 / t414;
            let t417 = v_sigma2 * t416;
            let t418 = t49 * t417;
            let t420 = t164 * t155;
            let t422 = f64x8::splat(1.0) / t156 / t420;
            let t424 = t61 * t163 * t422;
            let t426 = -f64x8::splat(0.40121303703703703) * t418 - f64x8::splat(0.014346394913206906) * t424;
            let t430 = v_tau1 * t159;
            let t437 = -f64x8::splat(0.17051554074074074) * t418 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t430 - f64x8::splat(0.031646617777777775) * t417) * t44 * t48;
            let t441 = f64x8::splat(1.0) / t185 / t171;
            let t442 = t184 * t441;
            let t445 = -t413 * t426 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t437 * t186 - f64x8::splat(14.0) / f64x8::splat(45.0) * t442 * t426;
            let t449 = f64x8::splat(6.0) * t407 * t400 - t403;
            let t451 = t222 * t222;
            let t452 = t451 * t451;
            let t453 = t452 * t452;
            let t454 = t453 * t222;
            let t455 = f64x8::splat(1.0) / t454;
            let t456 = t191 * t455;
            let t463 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t430 + t417 / f64x8::splat(3.0);
            let t464 = t463 * t44;
            let t465 = t48 * t210;
            let t469 = f64x8::splat(1.0) / t209 / t208;
            let t470 = t204 * t469;
            let t473 = t200 * t58;
            let t474 = t60 * t463;
            let t477 = f64x8::splat(0.2222222222222222) * t464 * t205 + f64x8::splat(0.12345679012345678) * t473 * t474;
            let t481 = t464 * t465 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t470 * t477 - f64x8::splat(2.0) / f64x8::splat(27.0) * t418;
            let t484 = t481 * t146;
            let t487 = t214 * t397;
            let t492 = -f64x8::splat(125.0) / f64x8::splat(39366.0) * t424 - f64x8::splat(10.0) / f64x8::splat(9.0) * t194 * t195 * t416 + f64x8::splat(584.0) / f64x8::splat(405.0) * t214 * t481 - f64x8::splat(146.0) / f64x8::splat(135.0) * t484 * t218 - f64x8::splat(146.0) / f64x8::splat(135.0) * t487 * t218 + f64x8::splat(146.0) / f64x8::splat(135.0) * t217 * t397;
            let t495 = t403 * t189 - f64x8::splat(6.0) * t407 * t409 + t154 * t445 + t449 * t222 + t456 * t492 / f64x8::splat(10.0);
            let t500 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t389 * t224 - t368 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t139 * t495));
            let tvrho1 = t129 + t228 + t7 * (t383 + t500);
            acc_vrho_1 = tvrho1;
            let t505 = ((t34).select(t29 * t31 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t506 = t35 * t505;
            let t508 = t36 * t505;
            let t510 = f64x8::splat(2.0) * t506 + f64x8::splat(9.0) * t508;
            let t511 = t510 * t42;
            let t513 = t262 * t505;
            let t516 = t49 * t54;
            let t519 = t61 * v_sigma0 * t66;
            let t521 = f64x8::splat(0.1504548888888889) * t516 + f64x8::splat(0.00537989809245259) * t519;
            let t529 = -t267 * t521 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t49 * t54 * t87 - f64x8::splat(14.0) / f64x8::splat(45.0) * t296 * t521;
            let t533 = f64x8::splat(6.0) * t261 * t508 - t511;
            let t540 = t49 * t54 * t111;
            let t542 = t54 * t44;
            let t543 = t542 * t106;
            let t546 = t327 * t60 * t54;
            let t548 = -f64x8::splat(0.027777777777777776) * t543 - f64x8::splat(0.015432098765432098) * t546;
            let t552 = -t540 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t324 * t548 + t516 / f64x8::splat(36.0);
            let t555 = t552 * t35;
            let t558 = t115 * t505;
            let t563 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t519 + f64x8::splat(5.0) / f64x8::splat(12.0) * t95 * t48 * t54 + f64x8::splat(584.0) / f64x8::splat(405.0) * t115 * t552 - f64x8::splat(146.0) / f64x8::splat(135.0) * t555 * t119 - f64x8::splat(146.0) / f64x8::splat(135.0) * t558 * t119 + f64x8::splat(146.0) / f64x8::splat(135.0) * t118 * t505;
            let t566 = t511 * t90 - f64x8::splat(6.0) * t261 * t513 + t43 * t529 + t533 * t123 + t310 * t563 / f64x8::splat(10.0);
            let t570 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t566));
            let tvsigma0 = t7 * t570;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t573 = ((t145).select(t140 * t142 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t574 = t146 * t573;
            let t576 = t147 * t573;
            let t578 = f64x8::splat(2.0) * t574 + f64x8::splat(9.0) * t576;
            let t579 = t578 * t153;
            let t581 = t408 * t573;
            let t584 = t49 * t159;
            let t587 = t61 * v_sigma2 * t167;
            let t589 = f64x8::splat(0.1504548888888889) * t584 + f64x8::splat(0.00537989809245259) * t587;
            let t597 = -t413 * t589 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t49 * t159 * t186 - f64x8::splat(14.0) / f64x8::splat(45.0) * t442 * t589;
            let t601 = f64x8::splat(6.0) * t407 * t576 - t579;
            let t608 = t49 * t159 * t210;
            let t610 = t159 * t44;
            let t611 = t610 * t205;
            let t614 = t473 * t60 * t159;
            let t616 = -f64x8::splat(0.027777777777777776) * t611 - f64x8::splat(0.015432098765432098) * t614;
            let t620 = -t608 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t470 * t616 + t584 / f64x8::splat(36.0);
            let t623 = t620 * t146;
            let t626 = t214 * t573;
            let t631 = f64x8::splat(125.0) / f64x8::splat(104976.0) * t587 + f64x8::splat(5.0) / f64x8::splat(12.0) * t194 * t48 * t159 + f64x8::splat(584.0) / f64x8::splat(405.0) * t214 * t620 - f64x8::splat(146.0) / f64x8::splat(135.0) * t623 * t218 - f64x8::splat(146.0) / f64x8::splat(135.0) * t626 * t218 + f64x8::splat(146.0) / f64x8::splat(135.0) * t217 * t573;
            let t634 = t579 * t189 - f64x8::splat(6.0) * t407 * t581 + t154 * t597 + t601 * t222 + t456 * t631 / f64x8::splat(10.0);
            let t638 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t139 * t634));
            let tvsigma2 = t7 * t638;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t639 = v_tau0 * v_tau0;
            let t640 = f64x8::splat(1.0) / t639;
            let t643 = ((t34).select(-t30 * t640 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t644 = t35 * t643;
            let t646 = t36 * t643;
            let t648 = f64x8::splat(2.0) * t644 + f64x8::splat(9.0) * t646;
            let t649 = t648 * t42;
            let t651 = t262 * t643;
            let t654 = t43 * t75;
            let t655 = t49 * t87;
            let t660 = f64x8::splat(6.0) * t261 * t646 - t649;
            let t662 = t75 * t44;
            let t670 = f64x8::splat(0.2222222222222222) * t662 * t106 + f64x8::splat(0.12345679012345678) * t327 * t60 * t75;
            let t673 = t662 * t319 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t324 * t670;
            let t676 = t673 * t35;
            let t679 = t115 * t643;
            let t684 = f64x8::splat(584.0) / f64x8::splat(405.0) * t115 * t673 - f64x8::splat(146.0) / f64x8::splat(135.0) * t676 * t119 - f64x8::splat(146.0) / f64x8::splat(135.0) * t679 * t119 + f64x8::splat(146.0) / f64x8::splat(135.0) * t118 * t643;
            let t687 = t649 * t90 - f64x8::splat(6.0) * t261 * t651 - f64x8::splat(0.06288822469135802) * t654 * t655 + t660 * t123 + t310 * t684 / f64x8::splat(10.0);
            let t691 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t687));
            let tvtau0 = t7 * t691;
            acc_vtau_0 = tvtau0;
            let t692 = v_tau1 * v_tau1;
            let t693 = f64x8::splat(1.0) / t692;
            let t696 = ((t145).select(-t141 * t693 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t697 = t146 * t696;
            let t699 = t147 * t696;
            let t701 = f64x8::splat(2.0) * t697 + f64x8::splat(9.0) * t699;
            let t702 = t701 * t153;
            let t704 = t408 * t696;
            let t707 = t154 * t176;
            let t708 = t49 * t186;
            let t713 = f64x8::splat(6.0) * t407 * t699 - t702;
            let t715 = t176 * t44;
            let t723 = f64x8::splat(0.2222222222222222) * t715 * t205 + f64x8::splat(0.12345679012345678) * t473 * t60 * t176;
            let t726 = t715 * t465 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t470 * t723;
            let t729 = t726 * t146;
            let t732 = t214 * t696;
            let t737 = f64x8::splat(584.0) / f64x8::splat(405.0) * t214 * t726 - f64x8::splat(146.0) / f64x8::splat(135.0) * t729 * t218 - f64x8::splat(146.0) / f64x8::splat(135.0) * t732 * t218 + f64x8::splat(146.0) / f64x8::splat(135.0) * t217 * t696;
            let t740 = t702 * t189 - f64x8::splat(6.0) * t407 * t704 - f64x8::splat(0.06288822469135802) * t707 * t708 + t713 * t222 + t456 * t737 / f64x8::splat(10.0);
            let t744 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t139 * t740));
            let tvtau1 = t7 * t744;
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
