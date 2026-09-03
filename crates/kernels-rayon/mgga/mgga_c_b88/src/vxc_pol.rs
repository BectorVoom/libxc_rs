//! MGGA_C_B88 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`
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
pub fn mgga_c_b88_vxc_pol(
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
            let t2 = v_rho0 - v_rho1;
            let t3 = t2 * t2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = f64x8::splat(1.0) / t5;
            let t8 = -t3 * t6 + f64x8::splat(1.0);
            let t9 = t8 * t4;
            let t10 = (v_rho0).simd_le(dens_threshold);
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = t11 * t11;
            let t13 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t14 = (simd::cbrt(t13));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t12 * t15;
            let t17 = f64x8::splat(M_CBRT4);
            let t18 = t16 * t17;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = f64x8::splat(1.0) / t4;
            let t23 = (f64x8::splat(2.0) * v_rho0 * t20).simd_le(zeta_threshold);
            let t24 = zeta_threshold - f64x8::splat(1.0);
            let t27 = (f64x8::splat(2.0) * v_rho1 * t20).simd_le(zeta_threshold);
            let t28 = -t24;
            let t29 = t2 * t20;
            let t30 = ((t23).select(t24, (t27).select(t28, t29)));
            let t31 = f64x8::splat(1.0) + t30;
            let t32 = t31 * t4;
            let t33 = (simd::cbrt(t32));
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t19 * t34;
            let t36 = v_rho0 * v_rho0;
            let t37 = (simd::cbrt(v_rho0));
            let t38 = t37 * t37;
            let t40 = f64x8::splat(1.0) / t38 / t36;
            let t41 = v_sigma0 * t40;
            let t43 = f64x8::splat(1.0) + f64x8::splat(0.007) * t41;
            let t44 = (simd::pow(t43, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t45 = t44 * t44;
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0008333333333333334) * t18 * t41 * t47;
            let t52 = f64x8::splat(1.0) / t51;
            let t54 = t18 * t35 * t52;
            let t56 = ((t10).select(f64x8::splat(0.0), t54 / f64x8::splat(9.0)));
            let t57 = f64x8::splat(0.63) * t56;
            let t58 = (v_rho1).simd_le(dens_threshold);
            let t59 = -t2;
            let t61 = ((t27).select(t24, (t23).select(t28, t59 * t20)));
            let t62 = f64x8::splat(1.0) + t61;
            let t63 = t62 * t4;
            let t64 = (simd::cbrt(t63));
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t19 * t65;
            let t67 = v_rho1 * v_rho1;
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t68 * t68;
            let t71 = f64x8::splat(1.0) / t69 / t67;
            let t72 = v_sigma2 * t71;
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.007) * t72;
            let t75 = (simd::pow(t74, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t76 = t75 * t75;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) / t77;
            let t82 = f64x8::splat(1.0) + f64x8::splat(0.0008333333333333334) * t18 * t72 * t78;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t18 * t66 * t83;
            let t87 = ((t58).select(f64x8::splat(0.0), t85 / f64x8::splat(9.0)));
            let t88 = f64x8::splat(0.63) * t87;
            let t89 = t57 + t88;
            let t90 = f64x8::splat(1.0) + t57 + t88;
            let t91 = (simd::ln(t90));
            let t92 = t57 + t88 - t91;
            let t93 = t89 * t92;
            let t95 = f64x8::splat(0.2) * t9 * t93;
            let t97 = (f64x8::splat(1.0) + t29).simd_le(zeta_threshold);
            let t99 = (f64x8::splat(1.0) - t29).simd_le(zeta_threshold);
            let t100 = ((t97).select(t24, (t99).select(t28, t29)));
            let t101 = f64x8::splat(1.0) + t100;
            let t102 = t101 * t101;
            let t103 = (simd::cbrt(t101));
            let t104 = t103 * t103;
            let t106 = t19 * t19;
            let t107 = t104 * t102 * t106;
            let t108 = (simd::cbrt(t4));
            let t109 = t108 * t108;
            let t110 = t109 * t4;
            let t112 = f64x8::splat(1.0) / t38 / v_rho0;
            let t116 = f64x8::splat(2.0) * v_tau0 * t112 - t41 / f64x8::splat(4.0);
            let t118 = t110 * t116 * t12;
            let t119 = t107 * t118;
            let t121 = f64x8::splat(1.0) / t14 / t13;
            let t122 = t121 * t17;
            let t124 = f64x8::splat(1.0) / t33 / t32;
            let t125 = t51 * t51;
            let t126 = t125 * t125;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t124 * t127;
            let t130 = f64x8::splat(1.0) + f64x8::splat(0.10666666666666667) * t54;
            let t131 = (simd::ln(t130));
            let t132 = t131 * t11;
            let t133 = t132 * t14;
            let t134 = t17 * t17;
            let t135 = t134 * t106;
            let t136 = t33 * t51;
            let t137 = t135 * t136;
            let t140 = f64x8::splat(1.0) - f64x8::splat(0.390625) * t133 * t137;
            let t142 = t122 * t128 * t140;
            let t145 = ((t10).select(f64x8::splat(0.0), -f64x8::splat(0.0001864135111111111) * t119 * t142));
            let t146 = ((t99).select(t24, (t97).select(t28, -t29)));
            let t147 = f64x8::splat(1.0) + t146;
            let t148 = t147 * t147;
            let t149 = (simd::cbrt(t147));
            let t150 = t149 * t149;
            let t152 = t150 * t148 * t106;
            let t154 = f64x8::splat(1.0) / t69 / v_rho1;
            let t158 = f64x8::splat(2.0) * v_tau1 * t154 - t72 / f64x8::splat(4.0);
            let t160 = t110 * t158 * t12;
            let t161 = t152 * t160;
            let t163 = f64x8::splat(1.0) / t64 / t63;
            let t164 = t82 * t82;
            let t165 = t164 * t164;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t163 * t166;
            let t169 = f64x8::splat(1.0) + f64x8::splat(0.10666666666666667) * t85;
            let t170 = (simd::ln(t169));
            let t171 = t170 * t11;
            let t172 = t171 * t14;
            let t173 = t64 * t82;
            let t174 = t135 * t173;
            let t177 = f64x8::splat(1.0) - f64x8::splat(0.390625) * t172 * t174;
            let t179 = t122 * t167 * t177;
            let t182 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(0.0001864135111111111) * t161 * t179));
            let tzk0 = -t95 + t145 + t182;
            acc_zk = tzk0;
            let t183 = t2 * t6;
            let t184 = t5 * t4;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t3 * t185;
            let t188 = -f64x8::splat(2.0) * t183 + f64x8::splat(2.0) * t186;
            let t189 = t188 * t4;
            let t190 = t189 * t93;
            let t191 = f64x8::splat(0.2) * t190;
            let t192 = t8 * t89;
            let t193 = t192 * t92;
            let t194 = f64x8::splat(0.2) * t193;
            let t195 = t19 * t124;
            let t196 = t20 - t183;
            let t197 = ((t23).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t196)));
            let t199 = t197 * t4 + t30 + f64x8::splat(1.0);
            let t200 = t52 * t199;
            let t202 = t18 * t195 * t200;
            let t204 = f64x8::splat(1.0) / t125;
            let t205 = t36 * v_rho0;
            let t207 = f64x8::splat(1.0) / t38 / t205;
            let t208 = v_sigma0 * t207;
            let t212 = v_sigma0 * v_sigma0;
            let t213 = t36 * t36;
            let t214 = t213 * t36;
            let t216 = f64x8::splat(1.0) / t37 / t214;
            let t219 = f64x8::splat(1.0) / t46 / t43;
            let t223 = -f64x8::splat(0.0022222222222222222) * t18 * t208 * t47 + f64x8::splat(1.2444444444444445e-05) * t18 * t212 * t216 * t219;
            let t224 = t204 * t223;
            let t226 = t18 * t35 * t224;
            let t229 = ((t10).select(f64x8::splat(0.0), -t202 / f64x8::splat(27.0) - t226 / f64x8::splat(9.0)));
            let t230 = f64x8::splat(0.63) * t229;
            let t231 = t19 * t163;
            let t232 = t59 * t6;
            let t234 = ((t27).select(f64x8::splat(0.0), (t23).select(f64x8::splat(0.0), -t20 - t232)));
            let t236 = t234 * t4 + t61 + f64x8::splat(1.0);
            let t237 = t83 * t236;
            let t241 = ((t58).select(f64x8::splat(0.0), -t18 * t231 * t237 / f64x8::splat(27.0)));
            let t242 = f64x8::splat(0.63) * t241;
            let t243 = t230 + t242;
            let t244 = t243 * t92;
            let t245 = t9 * t244;
            let t246 = f64x8::splat(0.2) * t245;
            let t247 = f64x8::splat(1.0) / t90;
            let t249 = -t243 * t247 + t230 + t242;
            let t250 = t89 * t249;
            let t251 = t9 * t250;
            let t252 = f64x8::splat(0.2) * t251;
            let t254 = t104 * t101 * t106;
            let t255 = t254 * t118;
            let t256 = t122 * t124;
            let t257 = t127 * t140;
            let t258 = ((t97).select(f64x8::splat(0.0), (t99).select(f64x8::splat(0.0), t196)));
            let t260 = t256 * t257 * t258;
            let t264 = t109 * t116 * t12;
            let t265 = t107 * t264;
            let t267 = f64x8::splat(0.00031068918518518517) * t265 * t142;
            let t271 = -f64x8::splat(10.0) / f64x8::splat(3.0) * v_tau0 * t40 + f64x8::splat(2.0) / f64x8::splat(3.0) * t208;
            let t273 = t110 * t271 * t12;
            let t274 = t107 * t273;
            let t277 = t31 * t31;
            let t278 = t277 * t5;
            let t280 = f64x8::splat(1.0) / t33 / t278;
            let t281 = t122 * t280;
            let t283 = t281 * t257 * t199;
            let t287 = f64x8::splat(1.0) / t126 / t51;
            let t288 = t287 * t140;
            let t290 = t256 * t288 * t223;
            let t295 = -f64x8::splat(0.035555555555555556) * t202 - f64x8::splat(0.10666666666666667) * t226;
            let t296 = f64x8::splat(1.0) / t130;
            let t297 = t295 * t296;
            let t298 = t11 * t14;
            let t299 = t297 * t298;
            let t302 = t14 * t134;
            let t303 = t132 * t302;
            let t304 = t33 * t33;
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t106 * t305;
            let t307 = t51 * t199;
            let t311 = t33 * t223;
            let t312 = t135 * t311;
            let t315 = -f64x8::splat(0.390625) * t299 * t137 - f64x8::splat(0.13020833333333334) * t303 * t306 * t307 - f64x8::splat(0.390625) * t133 * t312;
            let t317 = t122 * t128 * t315;
            let t321 = ((t10).select(f64x8::splat(0.0), -f64x8::splat(0.0004971026962962963) * t255 * t260 - t267 - f64x8::splat(0.0001864135111111111) * t274 * t142 + f64x8::splat(0.0002485513481481481) * t119 * t283 + f64x8::splat(0.0007456540444444444) * t119 * t290 - f64x8::splat(0.0001864135111111111) * t119 * t317));
            let t323 = t150 * t147 * t106;
            let t324 = t323 * t160;
            let t325 = t122 * t163;
            let t326 = t166 * t177;
            let t328 = ((t99).select(f64x8::splat(0.0), (t97).select(f64x8::splat(0.0), -t196)));
            let t330 = t325 * t326 * t328;
            let t334 = t109 * t158 * t12;
            let t335 = t152 * t334;
            let t337 = f64x8::splat(0.00031068918518518517) * t335 * t179;
            let t338 = t62 * t62;
            let t339 = t338 * t5;
            let t341 = f64x8::splat(1.0) / t64 / t339;
            let t342 = t122 * t341;
            let t344 = t342 * t326 * t236;
            let t347 = f64x8::splat(1.0) / t62;
            let t348 = t347 * t20;
            let t349 = f64x8::splat(1.0) / t169;
            let t350 = t236 * t349;
            let t353 = t171 * t302;
            let t354 = t64 * t64;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t106 * t355;
            let t357 = t82 * t236;
            let t361 = f64x8::splat(0.3333333333333333) * t348 * t350 - f64x8::splat(0.13020833333333334) * t353 * t356 * t357;
            let t363 = t122 * t167 * t361;
            let t367 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(0.0004971026962962963) * t324 * t330 - t337 + f64x8::splat(0.0002485513481481481) * t161 * t344 - f64x8::splat(0.0001864135111111111) * t161 * t363));
            let tvrho0 = -t95 + t145 + t182 + t4 * (-t191 - t194 - t246 - t252 + t321 + t367);
            acc_vrho_0 = tvrho0;
            let t371 = f64x8::splat(2.0) * t183 + f64x8::splat(2.0) * t186;
            let t372 = t371 * t4;
            let t373 = t372 * t93;
            let t374 = f64x8::splat(0.2) * t373;
            let t375 = -t20 - t183;
            let t376 = ((t23).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t375)));
            let t378 = t376 * t4 + t30 + f64x8::splat(1.0);
            let t379 = t52 * t378;
            let t383 = ((t10).select(f64x8::splat(0.0), -t18 * t195 * t379 / f64x8::splat(27.0)));
            let t384 = f64x8::splat(0.63) * t383;
            let t386 = ((t27).select(f64x8::splat(0.0), (t23).select(f64x8::splat(0.0), t20 - t232)));
            let t388 = t386 * t4 + t61 + f64x8::splat(1.0);
            let t391 = t18 * t231 * t83 * t388;
            let t393 = f64x8::splat(1.0) / t164;
            let t394 = t67 * v_rho1;
            let t396 = f64x8::splat(1.0) / t69 / t394;
            let t397 = v_sigma2 * t396;
            let t401 = v_sigma2 * v_sigma2;
            let t402 = t67 * t67;
            let t403 = t402 * t67;
            let t405 = f64x8::splat(1.0) / t68 / t403;
            let t408 = f64x8::splat(1.0) / t77 / t74;
            let t412 = -f64x8::splat(0.0022222222222222222) * t18 * t397 * t78 + f64x8::splat(1.2444444444444445e-05) * t18 * t401 * t405 * t408;
            let t415 = t18 * t66 * t393 * t412;
            let t418 = ((t58).select(f64x8::splat(0.0), -t391 / f64x8::splat(27.0) - t415 / f64x8::splat(9.0)));
            let t419 = f64x8::splat(0.63) * t418;
            let t420 = t384 + t419;
            let t421 = t420 * t92;
            let t422 = t9 * t421;
            let t423 = f64x8::splat(0.2) * t422;
            let t425 = -t420 * t247 + t384 + t419;
            let t426 = t89 * t425;
            let t427 = t9 * t426;
            let t428 = f64x8::splat(0.2) * t427;
            let t429 = ((t97).select(f64x8::splat(0.0), (t99).select(f64x8::splat(0.0), t375)));
            let t431 = t256 * t257 * t429;
            let t435 = t281 * t257 * t378;
            let t438 = f64x8::splat(1.0) / t31;
            let t439 = t438 * t20;
            let t440 = t378 * t296;
            let t443 = t51 * t378;
            let t447 = f64x8::splat(0.3333333333333333) * t439 * t440 - f64x8::splat(0.13020833333333334) * t303 * t306 * t443;
            let t449 = t122 * t128 * t447;
            let t453 = ((t10).select(f64x8::splat(0.0), -f64x8::splat(0.0004971026962962963) * t255 * t431 - t267 + f64x8::splat(0.0002485513481481481) * t119 * t435 - f64x8::splat(0.0001864135111111111) * t119 * t449));
            let t455 = ((t99).select(f64x8::splat(0.0), (t97).select(f64x8::splat(0.0), -t375)));
            let t457 = t325 * t326 * t455;
            let t463 = -f64x8::splat(10.0) / f64x8::splat(3.0) * v_tau1 * t71 + f64x8::splat(2.0) / f64x8::splat(3.0) * t397;
            let t465 = t110 * t463 * t12;
            let t466 = t152 * t465;
            let t470 = t342 * t326 * t388;
            let t474 = f64x8::splat(1.0) / t165 / t82;
            let t475 = t474 * t177;
            let t477 = t325 * t475 * t412;
            let t482 = -f64x8::splat(0.035555555555555556) * t391 - f64x8::splat(0.10666666666666667) * t415;
            let t483 = t482 * t349;
            let t484 = t483 * t298;
            let t487 = t82 * t388;
            let t491 = t64 * t412;
            let t492 = t135 * t491;
            let t495 = -f64x8::splat(0.390625) * t484 * t174 - f64x8::splat(0.13020833333333334) * t353 * t356 * t487 - f64x8::splat(0.390625) * t172 * t492;
            let t497 = t122 * t167 * t495;
            let t501 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(0.0004971026962962963) * t324 * t457 - t337 - f64x8::splat(0.0001864135111111111) * t466 * t179 + f64x8::splat(0.0002485513481481481) * t161 * t470 + f64x8::splat(0.0007456540444444444) * t161 * t477 - f64x8::splat(0.0001864135111111111) * t161 * t497));
            let tvrho1 = -t95 + t145 + t182 + t4 * (-t374 - t194 - t423 - t428 + t453 + t501);
            acc_vrho_1 = tvrho1;
            let t508 = t213 * v_rho0;
            let t510 = f64x8::splat(1.0) / t37 / t508;
            let t515 = f64x8::splat(0.0008333333333333334) * t16 * t17 * t40 * t47 - f64x8::splat(4.666666666666666e-06) * t18 * v_sigma0 * t510 * t219;
            let t516 = t204 * t515;
            let t520 = ((t10).select(f64x8::splat(0.0), -t18 * t35 * t516 / f64x8::splat(9.0)));
            let t521 = t520 * t92;
            let t523 = f64x8::splat(0.126) * t9 * t521;
            let t527 = f64x8::splat(0.63) * t520 - f64x8::splat(0.63) * t520 * t247;
            let t528 = t89 * t527;
            let t530 = f64x8::splat(0.2) * t9 * t528;
            let t532 = t110 * t40 * t12;
            let t533 = t107 * t532;
            let t534 = t533 * t142;
            let t537 = t256 * t288 * t515;
            let t540 = t52 * t515;
            let t543 = t33 * t515;
            let t544 = t135 * t543;
            let t547 = f64x8::splat(1.0) * t540 * t296 - f64x8::splat(0.390625) * t133 * t544;
            let t549 = t122 * t128 * t547;
            let t553 = ((t10).select(f64x8::splat(0.0), f64x8::splat(4.660337777777778e-05) * t534 + f64x8::splat(0.0007456540444444444) * t119 * t537 - f64x8::splat(0.0001864135111111111) * t119 * t549));
            let tvsigma0 = t4 * (-t523 - t530 + t553);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t559 = t402 * v_rho1;
            let t561 = f64x8::splat(1.0) / t68 / t559;
            let t566 = f64x8::splat(0.0008333333333333334) * t16 * t17 * t71 * t78 - f64x8::splat(4.666666666666666e-06) * t18 * v_sigma2 * t561 * t408;
            let t567 = t393 * t566;
            let t571 = ((t58).select(f64x8::splat(0.0), -t18 * t66 * t567 / f64x8::splat(9.0)));
            let t572 = t571 * t92;
            let t574 = f64x8::splat(0.126) * t9 * t572;
            let t578 = f64x8::splat(0.63) * t571 - f64x8::splat(0.63) * t571 * t247;
            let t579 = t89 * t578;
            let t581 = f64x8::splat(0.2) * t9 * t579;
            let t583 = t110 * t71 * t12;
            let t584 = t152 * t583;
            let t585 = t584 * t179;
            let t588 = t325 * t475 * t566;
            let t591 = t83 * t566;
            let t594 = t64 * t566;
            let t595 = t135 * t594;
            let t598 = f64x8::splat(1.0) * t591 * t349 - f64x8::splat(0.390625) * t172 * t595;
            let t600 = t122 * t167 * t598;
            let t604 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.660337777777778e-05) * t585 + f64x8::splat(0.0007456540444444444) * t161 * t588 - f64x8::splat(0.0001864135111111111) * t161 * t600));
            let tvsigma2 = t4 * (-t574 - t581 + t604);
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t607 = t110 * t112 * t12;
            let t608 = t107 * t607;
            let t611 = ((t10).select(f64x8::splat(0.0), -f64x8::splat(0.0003728270222222222) * t608 * t142));
            let tvtau0 = t4 * t611;
            acc_vtau_0 = tvtau0;
            let t613 = t110 * t154 * t12;
            let t614 = t152 * t613;
            let t617 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(0.0003728270222222222) * t614 * t179));
            let tvtau1 = t4 * t617;
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
