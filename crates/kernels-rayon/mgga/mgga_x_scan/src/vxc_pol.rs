//! MGGA_X_SCAN vxc pol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
    for k in 0..8 {
        let p = (ip + k).min(np - 1);
        b[k] = s[p * stride + offset];
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    for k in 0..m {
        s[(ip + k) * stride + offset] = a[k];
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_scan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = v_sigma0 * t39;
            let t41 = t34 * t40;
            let t45 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t46 = t29 * t29;
            let t47 = t45 * t46;
            let t48 = t31 * t30;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t35 * t35;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t36 / t53;
            let t56 = t51 * t55;
            let t57 = t45 * t29;
            let t58 = t33 * v_sigma0;
            let t59 = t58 * t39;
            let t62 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t59));
            let t66 = ((f64x8::splat(146.0)).sqrt());
            let t67 = t66 * t29;
            let t70 = t37 * v_rho0;
            let t71 = f64x8::splat(1.0) / t70;
            let t77 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau0 * t71 - t40 / f64x8::splat(8.0)) * t29 * t33;
            let t78 = f64x8::splat(1.0) - t77;
            let t80 = t78 * t78;
            let t82 = (simd::exp(-t80 / f64x8::splat(2.0)));
            let t85 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t78 * t82 / f64x8::splat(100.0);
            let t86 = t85 * t85;
            let t87 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t86;
            let t92 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t87);
            let t93 = (t77).simd_le(f64x8::splat(1.0));
            let t94 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t97 = t94 / (-t94 + param_c1);
            let t98 = (-t97).simd_lt(t77);
            let t99 = (t77).simd_lt(-t97);
            let t100 = ((t99).select(t77, -t97));
            let t101 = param_c1 * t100;
            let t102 = f64x8::splat(1.0) - t100;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = (simd::exp(-t101 * t103));
            let t106 = ((t98).select(f64x8::splat(0.0), t105));
            let t107 = ((param_d).abs());
            let t110 = (simd::ln(f64x8::splat(f64::EPSILON) / t107));
            let t113 = (-t110 + param_c2) / t110;
            let t114 = (t77).simd_lt(-t113);
            let t115 = ((t114).select(-t113, t77));
            let t116 = f64x8::splat(1.0) - t115;
            let t119 = (simd::exp(param_c2 / t116));
            let t121 = ((t114).select(f64x8::splat(0.0), -param_d * t119));
            let t122 = ((t93).select(t106, t121));
            let t123 = f64x8::splat(1.0) - t122;
            let t126 = t92 * t123 + f64x8::splat(1.174) * t122;
            let t127 = t28 * t126;
            let t128 = ((f64x8::splat(3.0)).sqrt());
            let t129 = f64x8::splat(1.0) / t31;
            let t130 = t46 * t129;
            let t131 = ((v_sigma0).sqrt());
            let t132 = t36 * v_rho0;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t130 * t131 * t133;
            let t136 = ((t135).sqrt());
            let t140 = (simd::exp(-f64x8::splat(9.8958) * t128 / t136));
            let t141 = f64x8::splat(1.0) - t140;
            let t142 = t127 * t141;
            let t145 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t142));
            let t146 = (v_rho1).simd_le(dens_threshold);
            let t147 = -t17;
            let t149 = ((t15).select(t12, (t11).select(t16, t147 * t8)));
            let t150 = f64x8::splat(1.0) + t149;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(t150));
            let t154 = ((t151).select(t23, t152 * t150));
            let t155 = t6 * t154;
            let t156 = v_rho1 * v_rho1;
            let t157 = (simd::cbrt(v_rho1));
            let t158 = t157 * t157;
            let t159 = t158 * t156;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = v_sigma2 * t160;
            let t162 = t34 * t161;
            let t164 = v_sigma2 * v_sigma2;
            let t165 = t156 * t156;
            let t166 = t165 * v_rho1;
            let t168 = f64x8::splat(1.0) / t157 / t166;
            let t169 = t164 * t168;
            let t170 = t33 * v_sigma2;
            let t171 = t170 * t160;
            let t174 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t171));
            let t180 = t158 * v_rho1;
            let t181 = f64x8::splat(1.0) / t180;
            let t187 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t181 - t161 / f64x8::splat(8.0)) * t29 * t33;
            let t188 = f64x8::splat(1.0) - t187;
            let t190 = t188 * t188;
            let t192 = (simd::exp(-t190 / f64x8::splat(2.0)));
            let t195 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t171 + t66 * t188 * t192 / f64x8::splat(100.0);
            let t196 = t195 * t195;
            let t197 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t162 + t50 * t169 * t174 / f64x8::splat(576.0) + t196;
            let t202 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t197);
            let t203 = (t187).simd_le(f64x8::splat(1.0));
            let t204 = (-t97).simd_lt(t187);
            let t205 = (t187).simd_lt(-t97);
            let t206 = ((t205).select(t187, -t97));
            let t207 = param_c1 * t206;
            let t208 = f64x8::splat(1.0) - t206;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = (simd::exp(-t207 * t209));
            let t212 = ((t204).select(f64x8::splat(0.0), t211));
            let t213 = (t187).simd_lt(-t113);
            let t214 = ((t213).select(-t113, t187));
            let t215 = f64x8::splat(1.0) - t214;
            let t218 = (simd::exp(param_c2 / t215));
            let t220 = ((t213).select(f64x8::splat(0.0), -param_d * t218));
            let t221 = ((t203).select(t212, t220));
            let t222 = f64x8::splat(1.0) - t221;
            let t225 = t202 * t222 + f64x8::splat(1.174) * t221;
            let t226 = t28 * t225;
            let t227 = ((v_sigma2).sqrt());
            let t228 = t157 * v_rho1;
            let t229 = f64x8::splat(1.0) / t228;
            let t231 = t130 * t227 * t229;
            let t232 = ((t231).sqrt());
            let t236 = (simd::exp(-f64x8::splat(9.8958) * t128 / t232));
            let t237 = f64x8::splat(1.0) - t236;
            let t238 = t226 * t237;
            let t241 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t155 * t238));
            let tzk0 = t145 + t241;
            acc_zk = tzk0;
            let t242 = t7 * t7;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t17 * t243;
            let t246 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t244)));
            let t249 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t246));
            let t250 = t6 * t249;
            let t253 = t28 * t28;
            let t254 = f64x8::splat(1.0) / t253;
            let t255 = t254 * t126;
            let t256 = t255 * t141;
            let t258 = t27 * t256 / f64x8::splat(8.0);
            let t259 = param_k1 * param_k1;
            let t260 = t87 * t87;
            let t262 = t259 / t260;
            let t263 = t35 * v_rho0;
            let t265 = f64x8::splat(1.0) / t37 / t263;
            let t266 = v_sigma0 * t265;
            let t269 = t52 * t35;
            let t271 = f64x8::splat(1.0) / t36 / t269;
            let t276 = t45 * t45;
            let t277 = t30 * t30;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t276 * t278;
            let t280 = t51 * v_sigma0;
            let t281 = t52 * t52;
            let t282 = t281 * v_rho0;
            let t283 = f64x8::splat(1.0) / t282;
            let t294 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t39 + t266 / f64x8::splat(3.0);
            let t296 = t34 * t82;
            let t299 = t66 * t80;
            let t303 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t58 * t265 - t66 * t294 * t296 / f64x8::splat(180.0) + t299 * t294 * t296 / f64x8::splat(180.0);
            let t306 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t266 - t50 * t51 * t271 * t62 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t279 * t280 * t283 * t62 + f64x8::splat(2.0) * t85 * t303;
            let t307 = t306 * t123;
            let t309 = t294 * t29;
            let t311 = f64x8::splat(5.0) / f64x8::splat(9.0) * t309 * t33;
            let t312 = ((t99).select(t311, f64x8::splat(0.0)));
            let t315 = t102 * t102;
            let t316 = f64x8::splat(1.0) / t315;
            let t317 = t316 * t312;
            let t319 = -param_c1 * t312 * t103 - t101 * t317;
            let t320 = t319 * t105;
            let t321 = ((t98).select(f64x8::splat(0.0), t320));
            let t322 = param_d * param_c2;
            let t323 = t116 * t116;
            let t324 = f64x8::splat(1.0) / t323;
            let t325 = ((t114).select(f64x8::splat(0.0), t311));
            let t329 = ((t114).select(f64x8::splat(0.0), -t322 * t324 * t325 * t119));
            let t330 = ((t93).select(t321, t329));
            let t333 = t262 * t307 - t92 * t330 + f64x8::splat(1.174) * t330;
            let t334 = t28 * t333;
            let t335 = t334 * t141;
            let t338 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t339 = t338 * t338;
            let t340 = t339 * t339;
            let t341 = t340 * t338;
            let t342 = t341 * t26;
            let t344 = f64x8::splat(1.0) / t136 / t135;
            let t345 = t127 * t344;
            let t346 = t342 * t345;
            let t348 = f64x8::splat(1.0) / t36 / t35;
            let t351 = t130 * t131 * t348 * t140;
            let t355 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t250 * t142 - t258 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t335 - f64x8::splat(1.6891736332904388) * t346 * t351));
            let t356 = t147 * t243;
            let t358 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t356)));
            let t361 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t152 * t358));
            let t362 = t6 * t361;
            let t365 = t254 * t225;
            let t366 = t365 * t237;
            let t368 = t155 * t366 / f64x8::splat(8.0);
            let t370 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t362 * t238 - t368));
            let tvrho0 = t145 + t241 + t7 * (t355 + t370);
            acc_vrho_0 = tvrho0;
            let t374 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t244)));
            let t377 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t374));
            let t378 = t6 * t377;
            let t382 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t378 * t142 - t258));
            let t384 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t356)));
            let t387 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t152 * t384));
            let t388 = t6 * t387;
            let t391 = t197 * t197;
            let t393 = t259 / t391;
            let t394 = t156 * v_rho1;
            let t396 = f64x8::splat(1.0) / t158 / t394;
            let t397 = v_sigma2 * t396;
            let t400 = t165 * t156;
            let t402 = f64x8::splat(1.0) / t157 / t400;
            let t407 = t164 * v_sigma2;
            let t408 = t165 * t165;
            let t409 = t408 * v_rho1;
            let t410 = f64x8::splat(1.0) / t409;
            let t421 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t160 + t397 / f64x8::splat(3.0);
            let t423 = t34 * t192;
            let t426 = t66 * t190;
            let t430 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t170 * t396 - t66 * t421 * t423 / f64x8::splat(180.0) + t426 * t421 * t423 / f64x8::splat(180.0);
            let t433 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t397 - t50 * t164 * t402 * t174 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t279 * t407 * t410 * t174 + f64x8::splat(2.0) * t195 * t430;
            let t434 = t433 * t222;
            let t436 = t421 * t29;
            let t438 = f64x8::splat(5.0) / f64x8::splat(9.0) * t436 * t33;
            let t439 = ((t205).select(t438, f64x8::splat(0.0)));
            let t442 = t208 * t208;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = t443 * t439;
            let t446 = -param_c1 * t439 * t209 - t207 * t444;
            let t447 = t446 * t211;
            let t448 = ((t204).select(f64x8::splat(0.0), t447));
            let t449 = t215 * t215;
            let t450 = f64x8::splat(1.0) / t449;
            let t451 = ((t213).select(f64x8::splat(0.0), t438));
            let t455 = ((t213).select(f64x8::splat(0.0), -t322 * t450 * t451 * t218));
            let t456 = ((t203).select(t448, t455));
            let t459 = t393 * t434 - t202 * t456 + f64x8::splat(1.174) * t456;
            let t460 = t28 * t459;
            let t461 = t460 * t237;
            let t464 = t341 * t154;
            let t466 = f64x8::splat(1.0) / t232 / t231;
            let t467 = t226 * t466;
            let t468 = t464 * t467;
            let t470 = f64x8::splat(1.0) / t157 / t156;
            let t473 = t130 * t227 * t470 * t236;
            let t477 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t388 * t238 - t368 - f64x8::splat(3.0) / f64x8::splat(8.0) * t155 * t461 - f64x8::splat(1.6891736332904388) * t468 * t473));
            let tvrho1 = t145 + t241 + t7 * (t382 + t477);
            acc_vrho_1 = tvrho1;
            let t481 = t39 * t29 * t33;
            let t487 = f64x8::splat(1.0) / t281;
            let t495 = t66 * t39;
            let t496 = t495 * t296;
            let t499 = t299 * t39 * t296;
            let t501 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t39 + t496 / f64x8::splat(1440.0) - t499 / f64x8::splat(1440.0);
            let t504 = f64x8::splat(5.0) / f64x8::splat(972.0) * t481 + t50 * v_sigma0 * t55 * t62 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t279 * t51 * t487 * t62 + f64x8::splat(2.0) * t85 * t501;
            let t505 = t504 * t123;
            let t507 = f64x8::splat(5.0) / f64x8::splat(72.0) * t481;
            let t508 = ((t99).select(-t507, f64x8::splat(0.0)));
            let t509 = param_c1 * t508;
            let t511 = t316 * t508;
            let t513 = -t101 * t511 - t509 * t103;
            let t514 = t513 * t105;
            let t515 = ((t98).select(f64x8::splat(0.0), t514));
            let t516 = ((t114).select(f64x8::splat(0.0), -t507));
            let t520 = ((t114).select(f64x8::splat(0.0), -t322 * t324 * t516 * t119));
            let t521 = ((t93).select(t515, t520));
            let t524 = t262 * t505 - t92 * t521 + f64x8::splat(1.174) * t521;
            let t525 = t28 * t524;
            let t526 = t525 * t141;
            let t529 = f64x8::splat(1.0) / t131;
            let t532 = t130 * t529 * t133 * t140;
            let t536 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t526 + f64x8::splat(0.6334401124839145) * t346 * t532));
            let tvsigma0 = t7 * t536;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t538 = t160 * t29 * t33;
            let t544 = f64x8::splat(1.0) / t408;
            let t552 = t66 * t160;
            let t553 = t552 * t423;
            let t556 = t426 * t160 * t423;
            let t558 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t160 + t553 / f64x8::splat(1440.0) - t556 / f64x8::splat(1440.0);
            let t561 = f64x8::splat(5.0) / f64x8::splat(972.0) * t538 + t50 * v_sigma2 * t168 * t174 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t279 * t164 * t544 * t174 + f64x8::splat(2.0) * t195 * t558;
            let t562 = t561 * t222;
            let t564 = f64x8::splat(5.0) / f64x8::splat(72.0) * t538;
            let t565 = ((t205).select(-t564, f64x8::splat(0.0)));
            let t566 = param_c1 * t565;
            let t568 = t443 * t565;
            let t570 = -t207 * t568 - t566 * t209;
            let t571 = t570 * t211;
            let t572 = ((t204).select(f64x8::splat(0.0), t571));
            let t573 = ((t213).select(f64x8::splat(0.0), -t564));
            let t577 = ((t213).select(f64x8::splat(0.0), -t322 * t450 * t573 * t218));
            let t578 = ((t203).select(t572, t577));
            let t581 = t393 * t562 - t202 * t578 + f64x8::splat(1.174) * t578;
            let t582 = t28 * t581;
            let t583 = t582 * t237;
            let t586 = f64x8::splat(1.0) / t227;
            let t589 = t130 * t586 * t229 * t236;
            let t593 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t155 * t583 + f64x8::splat(0.6334401124839145) * t468 * t589));
            let tvsigma2 = t7 * t593;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t594 = t66 * t71;
            let t599 = t299 * t71 * t296 / f64x8::splat(180.0) - t594 * t296 / f64x8::splat(180.0);
            let t600 = t85 * t599;
            let t606 = f64x8::splat(5.0) / f64x8::splat(9.0) * t71 * t29 * t33;
            let t607 = ((t99).select(t606, f64x8::splat(0.0)));
            let t608 = param_c1 * t607;
            let t612 = -t101 * t316 * t607 - t608 * t103;
            let t613 = t612 * t105;
            let t614 = ((t98).select(f64x8::splat(0.0), t613));
            let t615 = ((t114).select(f64x8::splat(0.0), t606));
            let t619 = ((t114).select(f64x8::splat(0.0), -t322 * t324 * t615 * t119));
            let t620 = ((t93).select(t614, t619));
            let t623 = f64x8::splat(2.0) * t262 * t600 * t123 - t92 * t620 + f64x8::splat(1.174) * t620;
            let t624 = t28 * t623;
            let t625 = t624 * t141;
            let t628 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t625));
            let tvtau0 = t7 * t628;
            acc_vtau_0 = tvtau0;
            let t629 = t66 * t181;
            let t634 = t426 * t181 * t423 / f64x8::splat(180.0) - t629 * t423 / f64x8::splat(180.0);
            let t635 = t195 * t634;
            let t641 = f64x8::splat(5.0) / f64x8::splat(9.0) * t181 * t29 * t33;
            let t642 = ((t205).select(t641, f64x8::splat(0.0)));
            let t643 = param_c1 * t642;
            let t647 = -t207 * t443 * t642 - t643 * t209;
            let t648 = t647 * t211;
            let t649 = ((t204).select(f64x8::splat(0.0), t648));
            let t650 = ((t213).select(f64x8::splat(0.0), t641));
            let t654 = ((t213).select(f64x8::splat(0.0), -t322 * t450 * t650 * t218));
            let t655 = ((t203).select(t649, t654));
            let t658 = f64x8::splat(2.0) * t393 * t635 * t222 - t202 * t655 + f64x8::splat(1.174) * t655;
            let t659 = t28 * t658;
            let t660 = t659 * t237;
            let t663 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t155 * t660));
            let tvtau1 = t7 * t663;
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
