//! GGA_X_PW91 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw91.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pw91_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_alpha = f64x8::splat(param_alpha);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_expo = f64x8::splat(param_expo);
    let param_f = f64x8::splat(param_f);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t37 = (simd::exp(-param_alpha * t20 * t25 * t34 / f64x8::splat(24.0)));
            let t40 = (param_d * t37 + param_c) * t20;
            let t41 = t40 * t25;
            let t44 = t20 * t20;
            let t45 = f64x8::splat(1.0) / t23;
            let t46 = t44 * t45;
            let t47 = ((v_sigma).sqrt());
            let t50 = f64x8::splat(1.0) / t18 / v_rho;
            let t51 = t47 * t27 * t50;
            let t54 = (simd::pow(t46 * t51 / f64x8::splat(12.0), param_expo));
            let t55 = param_f * t54;
            let t56 = t41 * t34 / f64x8::splat(24.0) - t55;
            let t57 = t46 * t47;
            let t63 = (simd::ln(param_b * t44 * t45 * t51 / f64x8::splat(12.0) + ((((param_b * t44 * t45 * t51 / f64x8::splat(12.0)) * (param_b * t44 * t45 * t51 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t64 = param_a * t63;
            let t65 = t27 * t50 * t64;
            let t68 = f64x8::splat(1.0) + t57 * t65 / f64x8::splat(12.0) + t55;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = t56 * t69 + f64x8::splat(1.0);
            let t75 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t71));
            let tzk0 = f64x8::splat(2.0) * t75;
            acc_zk = tzk0;
            let t77 = t17 / t31;
            let t81 = param_d * param_alpha;
            let t83 = f64x8::splat(1.0) / t23 / t22;
            let t84 = t44 * t83;
            let t85 = t81 * t84;
            let t86 = v_sigma * v_sigma;
            let t87 = t86 * t27;
            let t88 = t30 * t30;
            let t89 = t88 * t30;
            let t91 = f64x8::splat(1.0) / t18 / t89;
            let t92 = t91 * t37;
            let t96 = t30 * v_rho;
            let t98 = f64x8::splat(1.0) / t31 / t96;
            let t102 = f64x8::splat(1.0) / v_rho;
            let t105 = f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * param_expo * t102;
            let t106 = t85 * t87 * t92 / f64x8::splat(108.0) - t41 * t29 * t98 / f64x8::splat(9.0) + t105;
            let t108 = t68 * t68;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t56 * t109;
            let t114 = t27 / t18 / t30 * t64;
            let t117 = t20 * t25;
            let t118 = t117 * t29;
            let t120 = param_b * param_b;
            let t125 = f64x8::splat(6.0) * t120 * t20 * t25 * t34 + f64x8::splat(144.0);
            let t126 = ((t125).sqrt());
            let t128 = param_b / t126;
            let t129 = t98 * param_a * t128;
            let t132 = -t57 * t114 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t118 * t129 - t105;
            let t134 = t106 * t69 - t110 * t132;
            let t139 = ((t2).select(f64x8::splat(0.0), -t6 * t77 * t71 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t134));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t139 + f64x8::splat(2.0) * t75;
            acc_vrho = tvrho0;
            let t142 = t88 * v_rho;
            let t144 = f64x8::splat(1.0) / t18 / t142;
            let t145 = t27 * t144;
            let t146 = t37 * v_sigma;
            let t150 = t25 * t28;
            let t154 = f64x8::splat(1.0) / v_sigma;
            let t157 = t55 * param_expo * t154 / f64x8::splat(2.0);
            let t158 = -t85 * t145 * t146 / f64x8::splat(288.0) + t40 * t150 * t33 / f64x8::splat(24.0) - t157;
            let t161 = t46 / t47;
            let t164 = t117 * t28;
            let t166 = t33 * param_a * t128;
            let t169 = t161 * t65 / f64x8::splat(24.0) + t164 * t166 / f64x8::splat(4.0) + t157;
            let t171 = -t110 * t169 + t158 * t69;
            let t175 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t171));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t175;
            acc_vsigma = tvsigma0;
            let t180 = t17 / t31 / v_rho;
            let t187 = t88 * t96;
            let t189 = f64x8::splat(1.0) / t18 / t187;
            let t190 = t189 * t37;
            let t194 = param_alpha * param_alpha;
            let t195 = param_d * t194;
            let t196 = t22 * t22;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t195 * t197;
            let t199 = t86 * v_sigma;
            let t200 = t88 * t88;
            let t201 = t200 * t30;
            let t202 = f64x8::splat(1.0) / t201;
            let t208 = f64x8::splat(1.0) / t31 / t88;
            let t212 = param_expo * param_expo;
            let t213 = f64x8::splat(1.0) / t30;
            let t214 = t212 * t213;
            let t216 = f64x8::splat(16.0) / f64x8::splat(9.0) * t55 * t214;
            let t219 = f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * param_expo * t213;
            let t220 = -t85 * t87 * t190 / f64x8::splat(12.0) + t198 * t199 * t202 * t37 / f64x8::splat(81.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t41 * t29 * t208 - t216 - t219;
            let t222 = t106 * t109;
            let t226 = f64x8::splat(1.0) / t108 / t68;
            let t227 = t56 * t226;
            let t228 = t132 * t132;
            let t234 = t27 / t18 / t96 * t64;
            let t238 = t208 * param_a * t128;
            let t241 = t84 * t87;
            let t243 = t120 * param_b;
            let t245 = f64x8::splat(1.0) / t126 / t125;
            let t246 = t243 * t245;
            let t247 = t189 * param_a * t246;
            let t250 = f64x8::splat(7.0) / f64x8::splat(27.0) * t57 * t234 + f64x8::splat(10.0) / f64x8::splat(3.0) * t118 * t238 - f64x8::splat(32.0) / f64x8::splat(3.0) * t241 * t247 + t216 + t219;
            let t252 = -t110 * t250 - f64x8::splat(2.0) * t222 * t132 + t220 * t69 + f64x8::splat(2.0) * t227 * t228;
            let t257 = ((t2).select(f64x8::splat(0.0), t6 * t180 * t71 / f64x8::splat(12.0) - t6 * t77 * t134 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t252));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t257 + f64x8::splat(4.0) * t139;
            acc_v2rho2 = tv2rho20;
            let t263 = t27 * t91;
            let t267 = t200 * v_rho;
            let t268 = f64x8::splat(1.0) / t267;
            let t276 = t212 * t102;
            let t279 = f64x8::splat(2.0) / f64x8::splat(3.0) * t55 * t276 * t154;
            let t280 = t85 * t263 * t146 / f64x8::splat(36.0) - t198 * t268 * t86 * t37 / f64x8::splat(216.0) - t40 * t150 * t98 / f64x8::splat(9.0) + t279;
            let t282 = t158 * t109;
            let t285 = t169 * t132;
            let t294 = param_a * t243 * t245 * v_sigma;
            let t297 = -t161 * t114 / f64x8::splat(18.0) - t164 * t129 + f64x8::splat(4.0) * t84 * t263 * t294 - t279;
            let t299 = -t110 * t297 - t282 * t132 - t222 * t169 + f64x8::splat(2.0) * t227 * t285 + t280 * t69;
            let t304 = ((t2).select(f64x8::splat(0.0), -t6 * t77 * t171 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t299));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t304 + f64x8::splat(2.0) * t175;
            acc_v2rhosigma = tv2rhosigma0;
            let t307 = f64x8::splat(1.0) / t200;
            let t312 = t81 * t44;
            let t313 = t83 * t27;
            let t318 = f64x8::splat(1.0) / t86;
            let t321 = t55 * t212 * t318 / f64x8::splat(4.0);
            let t324 = t55 * param_expo * t318 / f64x8::splat(2.0);
            let t325 = t198 * t307 * t37 * v_sigma / f64x8::splat(576.0) - t312 * t313 * t144 * t37 / f64x8::splat(144.0) - t321 + t324;
            let t329 = t169 * t169;
            let t334 = t46 / t47 / v_sigma;
            let t338 = t117 * t154 * t28;
            let t341 = t84 * t27;
            let t343 = t144 * param_a * t246;
            let t346 = -t334 * t65 / f64x8::splat(48.0) + t338 * t166 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t341 * t343 + t321 - t324;
            let t348 = -t110 * t346 - f64x8::splat(2.0) * t282 * t169 + f64x8::splat(2.0) * t227 * t329 + t325 * t69;
            let t352 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t348));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t352;
            acc_v2sigma2 = tv2sigma20;
            let t355 = t17 * t33;
            let t366 = f64x8::splat(1.0) / t18 / t200;
            let t371 = t200 * t96;
            let t372 = f64x8::splat(1.0) / t371;
            let t378 = param_d * t194 * param_alpha;
            let t379 = t86 * t86;
            let t380 = t197 * t379;
            let t381 = t378 * t380;
            let t382 = t200 * t142;
            let t384 = f64x8::splat(1.0) / t31 / t382;
            let t386 = t150 * t37;
            let t391 = f64x8::splat(1.0) / t31 / t142;
            let t395 = t212 * param_expo;
            let t396 = f64x8::splat(1.0) / t96;
            let t397 = t395 * t396;
            let t399 = f64x8::splat(64.0) / f64x8::splat(27.0) * t55 * t397;
            let t400 = t212 * t396;
            let t402 = f64x8::splat(16.0) / f64x8::splat(3.0) * t55 * t400;
            let t405 = f64x8::splat(8.0) / f64x8::splat(3.0) * t55 * param_expo * t396;
            let t406 = f64x8::splat(341.0) / f64x8::splat(486.0) * t85 * t87 * t366 * t37 - f64x8::splat(19.0) / f64x8::splat(81.0) * t198 * t199 * t372 * t37 + t381 * t384 * t20 * t386 / f64x8::splat(729.0) - f64x8::splat(154.0) / f64x8::splat(81.0) * t41 * t29 * t391 + t399 + t402 + t405;
            let t408 = t220 * t109;
            let t411 = t106 * t226;
            let t416 = t108 * t108;
            let t417 = f64x8::splat(1.0) / t416;
            let t418 = t56 * t417;
            let t419 = t228 * t132;
            let t422 = t132 * t250;
            let t428 = t27 / t18 / t88 * t64;
            let t432 = t391 * param_a * t128;
            let t439 = t197 * t199;
            let t441 = t120 * t120;
            let t442 = t441 * param_b;
            let t444 = t125 * t125;
            let t446 = f64x8::splat(1.0) / t126 / t444;
            let t447 = param_a * t442 * t446;
            let t450 = -f64x8::splat(70.0) / f64x8::splat(81.0) * t57 * t428 - f64x8::splat(476.0) / f64x8::splat(27.0) * t118 * t432 + f64x8::splat(1184.0) / f64x8::splat(9.0) * t241 * t366 * param_a * t246 - f64x8::splat(3072.0) * t439 * t372 * t447 - t399 - t402 - t405;
            let t452 = -t110 * t450 - f64x8::splat(3.0) * t408 * t132 - f64x8::splat(3.0) * t222 * t250 + f64x8::splat(6.0) * t227 * t422 + f64x8::splat(6.0) * t411 * t228 + t406 * t69 - f64x8::splat(6.0) * t418 * t419;
            let t457 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t355 * t71 + t6 * t180 * t134 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t77 * t252 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t452));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t457 + f64x8::splat(6.0) * t257;
            acc_v3rho3 = tv3rho30;
            let t467 = t27 * t189;
            let t475 = t200 * t88;
            let t478 = t197 / t31 / t475;
            let t479 = t378 * t478;
            let t481 = t199 * t20 * t386;
            let t487 = t395 * t213;
            let t490 = f64x8::splat(8.0) / f64x8::splat(9.0) * t55 * t487 * t154;
            let t493 = f64x8::splat(2.0) / f64x8::splat(3.0) * t55 * t214 * t154;
            let t494 = -f64x8::splat(65.0) / f64x8::splat(324.0) * t85 * t467 * t146 + f64x8::splat(17.0) / f64x8::splat(216.0) * t198 * t202 * t86 * t37 - t479 * t481 / f64x8::splat(1944.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t40 * t150 * t208 - t490 - t493;
            let t496 = t280 * t109;
            let t499 = t158 * t226;
            let t508 = t169 * t228;
            let t511 = t297 * t132;
            let t514 = t169 * t250;
            let t525 = t197 * t202 * param_a;
            let t526 = t442 * t446;
            let t527 = t526 * t86;
            let t530 = f64x8::splat(7.0) / f64x8::splat(54.0) * t161 * t234 + f64x8::splat(37.0) / f64x8::splat(9.0) * t164 * t238 - f64x8::splat(124.0) / f64x8::splat(3.0) * t84 * t467 * t294 + f64x8::splat(1152.0) * t525 * t527 + t490 + t493;
            let t532 = -t110 * t530 - f64x8::splat(2.0) * t496 * t132 - t408 * t169 - f64x8::splat(2.0) * t222 * t297 + f64x8::splat(4.0) * t227 * t511 + f64x8::splat(2.0) * t227 * t514 + f64x8::splat(2.0) * t499 * t228 - t282 * t250 + f64x8::splat(4.0) * t411 * t285 - f64x8::splat(6.0) * t418 * t508 + t494 * t69;
            let t537 = ((t2).select(f64x8::splat(0.0), t6 * t180 * t171 / f64x8::splat(12.0) - t6 * t77 * t299 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t532));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t537 + f64x8::splat(4.0) * t304;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t549 = t197 / t31 / t371;
            let t550 = t378 * t549;
            let t553 = t117 * t86 * t28 * t37;
            let t559 = t395 * t102;
            let t562 = t55 * t559 * t318 / f64x8::splat(3.0);
            let t565 = f64x8::splat(2.0) / f64x8::splat(3.0) * t55 * t276 * t318;
            let t566 = -f64x8::splat(5.0) / f64x8::splat(216.0) * t198 * t268 * t37 * v_sigma + t550 * t553 / f64x8::splat(5184.0) + t312 * t313 * t92 / f64x8::splat(27.0) + t562 - t565;
            let t568 = t325 * t109;
            let t578 = t329 * t132;
            let t581 = t169 * t297;
            let t585 = t346 * t132;
            let t593 = t91 * param_a * t246;
            let t596 = t197 * t268;
            let t598 = t526 * v_sigma;
            let t601 = t334 * t114 / f64x8::splat(36.0) - t338 * t129 / f64x8::splat(6.0) + f64x8::splat(10.0) * t341 * t593 - f64x8::splat(432.0) * t596 * param_a * t598 - t562 + t565;
            let t603 = -t110 * t601 - t568 * t132 - f64x8::splat(2.0) * t496 * t169 - t222 * t346 + f64x8::splat(4.0) * t227 * t581 + f64x8::splat(2.0) * t227 * t585 - f64x8::splat(2.0) * t282 * t297 + f64x8::splat(4.0) * t499 * t285 + f64x8::splat(2.0) * t411 * t329 - f64x8::splat(6.0) * t418 * t578 + t566 * t69;
            let t608 = ((t2).select(f64x8::splat(0.0), -t6 * t77 * t348 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t603));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t608 + f64x8::splat(2.0) * t352;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t613 = t197 / t31 / t201;
            let t614 = t378 * t613;
            let t615 = t28 * t37;
            let t617 = t117 * t615 * v_sigma;
            let t620 = t197 * t307;
            let t624 = f64x8::splat(1.0) / t199;
            let t627 = t55 * t395 * t624 / f64x8::splat(8.0);
            let t630 = f64x8::splat(3.0) / f64x8::splat(4.0) * t55 * t212 * t624;
            let t632 = t55 * param_expo * t624;
            let t633 = -t614 * t617 / f64x8::splat(13824.0) + t195 * t620 * t37 / f64x8::splat(192.0) - t627 + t630 - t632;
            let t641 = t329 * t169;
            let t644 = t169 * t346;
            let t649 = t46 / t47 / t86;
            let t653 = t117 * t318 * t28;
            let t657 = t84 * t154 * t27;
            let t662 = t649 * t65 / f64x8::splat(32.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t653 * t166 - f64x8::splat(3.0) / f64x8::splat(4.0) * t657 * t343 + f64x8::splat(162.0) * t620 * t447 + t627 - t630 + t632;
            let t664 = -t110 * t662 - f64x8::splat(3.0) * t568 * t169 + f64x8::splat(6.0) * t227 * t644 - f64x8::splat(3.0) * t282 * t346 + f64x8::splat(6.0) * t499 * t329 - f64x8::splat(6.0) * t418 * t641 + t633 * t69;
            let t668 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t664));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t668;
            acc_v3sigma3 = tv3sigma30;
            let t685 = f64x8::splat(1.0) / t18 / t267;
            let t690 = f64x8::splat(1.0) / t475;
            let t695 = t200 * t89;
            let t697 = f64x8::splat(1.0) / t31 / t695;
            let t702 = t194 * t194;
            let t703 = param_d * t702;
            let t707 = t200 * t200;
            let t712 = t313 * t37;
            let t717 = f64x8::splat(1.0) / t31 / t89;
            let t721 = t212 * t212;
            let t722 = f64x8::splat(1.0) / t88;
            let t725 = f64x8::splat(256.0) / f64x8::splat(81.0) * t55 * t721 * t722;
            let t728 = f64x8::splat(128.0) / f64x8::splat(9.0) * t55 * t395 * t722;
            let t731 = f64x8::splat(176.0) / f64x8::splat(9.0) * t55 * t212 * t722;
            let t734 = f64x8::splat(8.0) * t55 * param_expo * t722;
            let t737 = t406 * t109;
            let t740 = t220 * t226;
            let t745 = t106 * t417;
            let t754 = t56 / t416 / t68;
            let t755 = t228 * t228;
            let t761 = t250 * t250;
            let t783 = t441 * t243;
            let t786 = f64x8::splat(1.0) / t126 / t444 / t125;
            let t793 = (-f64x8::splat(3047.0) / f64x8::splat(486.0) * t85 * t87 * t685 * t37 + f64x8::splat(2563.0) / f64x8::splat(729.0) * t198 * t199 * t690 * t37 - f64x8::splat(98.0) / f64x8::splat(2187.0) * t381 * t697 * t20 * t386 + f64x8::splat(2.0) / f64x8::splat(6561.0) * t703 * t197 * t379 * v_sigma / t18 / t707 / v_rho * t44 * t712 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t41 * t29 * t717 - t725 - t728 - t731 - t734) * t69 - f64x8::splat(4.0) * t737 * t132 + f64x8::splat(12.0) * t740 * t228 - f64x8::splat(6.0) * t408 * t250 - f64x8::splat(24.0) * t745 * t419 + f64x8::splat(24.0) * t411 * t422 - f64x8::splat(4.0) * t222 * t450 + f64x8::splat(24.0) * t754 * t755 - f64x8::splat(36.0) * t418 * t228 * t250 + f64x8::splat(6.0) * t227 * t761 + f64x8::splat(8.0) * t227 * t132 * t450 - t110 * (f64x8::splat(910.0) / f64x8::splat(243.0) * t57 * t145 * t64 + f64x8::splat(2884.0) / f64x8::splat(27.0) * t118 * t717 * param_a * t128 - f64x8::splat(37216.0) / f64x8::splat(27.0) * t241 * t685 * param_a * t246 + f64x8::splat(71680.0) * t439 * t690 * t447 - f64x8::splat(122880.0) * t380 * t697 * param_a * t783 * t786 * t164 + t725 + t728 + t731 + t734);
            let t798 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t98 * t71 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t355 * t134 + t6 * t180 * t252 / f64x8::splat(2.0) - t6 * t77 * t452 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t793));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t798 + f64x8::splat(8.0) * t457;
            acc_v4rho4 = tv4rho40;
            let t811 = t280 * t226;
            let t814 = t158 * t417;
            let t838 = t27 * t366;
            let t846 = t197 * t384;
            let t864 = f64x8::splat(32.0) / f64x8::splat(27.0) * t55 * t721 * t396 * t154;
            let t867 = f64x8::splat(8.0) / f64x8::splat(3.0) * t55 * t397 * t154;
            let t870 = f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t400 * t154;
            let t881 = t494 * t109;
            let t903 = param_a * t783;
            let t911 = (f64x8::splat(253.0) / f64x8::splat(162.0) * t85 * t838 * t146 - f64x8::splat(1025.0) / f64x8::splat(972.0) * t198 * t372 * t86 * t37 + f64x8::splat(89.0) / f64x8::splat(5832.0) * t378 * t846 * t481 - t703 * t197 / t18 / t707 * t379 * t44 * t712 / f64x8::splat(8748.0) - f64x8::splat(154.0) / f64x8::splat(81.0) * t40 * t150 * t391 + t864 + t867 + t870) * t69 + f64x8::splat(24.0) * t754 * t169 * t419 - f64x8::splat(18.0) * t418 * t285 * t250 - f64x8::splat(18.0) * t745 * t508 - f64x8::splat(3.0) * t881 * t132 - f64x8::splat(3.0) * t496 * t250 - t282 * t450 - t737 * t169 - f64x8::splat(3.0) * t408 * t297 - f64x8::splat(3.0) * t222 * t530 - t110 * (-f64x8::splat(35.0) / f64x8::splat(81.0) * t161 * t428 - f64x8::splat(182.0) / f64x8::splat(9.0) * t164 * t432 + f64x8::splat(3320.0) / f64x8::splat(9.0) * t84 * t838 * t294 - f64x8::splat(23424.0) * t197 * t372 * param_a * t527 + f64x8::splat(46080.0) * t846 * t903 * t786 * t199 * t164 - t864 - t867 - t870);
            let t917 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t355 * t171 + t6 * t180 * t299 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t77 * t532 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (f64x8::splat(6.0) * t227 * t530 * t132 + f64x8::splat(2.0) * t227 * t169 * t450 + f64x8::splat(6.0) * t227 * t297 * t250 - f64x8::splat(18.0) * t418 * t297 * t228 + f64x8::splat(6.0) * t811 * t228 + f64x8::splat(6.0) * t740 * t285 + f64x8::splat(12.0) * t411 * t511 + f64x8::splat(6.0) * t411 * t514 - f64x8::splat(6.0) * t814 * t419 + f64x8::splat(6.0) * t499 * t422 + t911)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t917 + f64x8::splat(6.0) * t537;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t949 = f64x8::splat(4.0) / f64x8::splat(9.0) * t55 * t721 * t213 * t318;
            let t952 = f64x8::splat(5.0) / f64x8::splat(9.0) * t55 * t487 * t318;
            let t955 = f64x8::splat(2.0) / f64x8::splat(3.0) * t55 * t214 * t318;
            let t988 = (f64x8::splat(167.0) / f64x8::splat(648.0) * t198 * t202 * t37 * v_sigma - f64x8::splat(25.0) / f64x8::splat(5184.0) * t479 * t553 + t703 * t197 / t18 / t200 / t187 * t84 * t199 * t27 * t37 / f64x8::splat(23328.0) - f64x8::splat(19.0) / f64x8::splat(81.0) * t312 * t313 * t190 - t949 + t952 + t955) * t69 - f64x8::splat(24.0) * t418 * t285 * t297 + f64x8::splat(24.0) * t754 * t329 * t228 - f64x8::splat(6.0) * t418 * t346 * t228 + f64x8::splat(8.0) * t811 * t285 + f64x8::splat(8.0) * t499 * t511 + f64x8::splat(4.0) * t499 * t514 - f64x8::splat(12.0) * t745 * t578 + f64x8::splat(8.0) * t411 * t581 - f64x8::splat(6.0) * t418 * t329 * t250 + f64x8::splat(4.0) * t227 * t169 * t530 + f64x8::splat(4.0) * t411 * t585 + f64x8::splat(4.0) * t227 * t601 * t132;
            let t994 = t297 * t297;
            let t997 = t566 * t109;
            let t1027 = t325 * t226;
            let t1030 = f64x8::splat(2.0) * t227 * t346 * t250 - f64x8::splat(12.0) * t814 * t508 + f64x8::splat(4.0) * t227 * t994 - f64x8::splat(2.0) * t997 * t132 - t568 * t250 - f64x8::splat(2.0) * t881 * t169 - f64x8::splat(4.0) * t496 * t297 - f64x8::splat(2.0) * t282 * t530 + f64x8::splat(2.0) * t740 * t329 - t408 * t346 - f64x8::splat(2.0) * t222 * t601 - t110 * (-f64x8::splat(7.0) / f64x8::splat(108.0) * t334 * t234 + f64x8::splat(7.0) / f64x8::splat(18.0) * t338 * t238 - f64x8::splat(66.0) * t341 * t247 + f64x8::splat(6768.0) * t525 * t598 - f64x8::splat(17280.0) * t478 * t903 * t786 * t86 * t164 + t949 - t952 - t955) + f64x8::splat(2.0) * t1027 * t228;
            let t1036 = ((t2).select(f64x8::splat(0.0), t6 * t180 * t348 / f64x8::splat(12.0) - t6 * t77 * t603 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t988 + t1030)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1036 + f64x8::splat(4.0) * t608;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1058 = t55 * t721 * t102 * t624 / f64x8::splat(6.0);
            let t1060 = t55 * t559 * t624;
            let t1063 = f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t276 * t624;
            let t1066 = t633 * t109;
            let t1118 = t786 * t20;
            let t1126 = -f64x8::splat(3.0) * t282 * t601 - f64x8::splat(6.0) * t745 * t641 + f64x8::splat(24.0) * t754 * t641 * t132 - f64x8::splat(18.0) * t418 * t329 * t297 + f64x8::splat(6.0) * t411 * t644 - f64x8::splat(18.0) * t418 * t644 * t132 + f64x8::splat(6.0) * t227 * t297 * t346 + f64x8::splat(6.0) * t227 * t169 * t601 - t222 * t662 + f64x8::splat(2.0) * t227 * t662 * t132 - t110 * (-t649 * t114 / f64x8::splat(24.0) + t653 * t129 / f64x8::splat(4.0) + t657 * t593 - f64x8::splat(1512.0) * t596 * t447 + f64x8::splat(6480.0) * t549 * t903 * t1118 * t25 * v_sigma * t28 - t1058 + t1060 - t1063);
            let t1132 = ((t2).select(f64x8::splat(0.0), -t6 * t77 * t664 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * ((f64x8::splat(7.0) / f64x8::splat(5184.0) * t550 * t617 - t703 * t197 / t18 / t695 * t84 * t87 * t37 / f64x8::splat(62208.0) - t195 * t596 * t37 / f64x8::splat(24.0) + t1058 - t1060 + t1063) * t69 - t1066 * t132 - f64x8::splat(3.0) * t997 * t169 + f64x8::splat(6.0) * t1027 * t285 - f64x8::splat(3.0) * t568 * t297 + f64x8::splat(6.0) * t811 * t329 - f64x8::splat(18.0) * t814 * t578 + f64x8::splat(12.0) * t499 * t581 - f64x8::splat(3.0) * t496 * t346 + f64x8::splat(6.0) * t499 * t585 + t1126)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t1132 + f64x8::splat(2.0) * t668;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1147 = f64x8::splat(1.0) / t379;
            let t1150 = t55 * t721 * t1147 / f64x8::splat(16.0);
            let t1153 = f64x8::splat(3.0) / f64x8::splat(4.0) * t55 * t395 * t1147;
            let t1156 = f64x8::splat(11.0) / f64x8::splat(4.0) * t55 * t212 * t1147;
            let t1159 = f64x8::splat(3.0) * t55 * param_expo * t1147;
            let t1174 = t329 * t329;
            let t1180 = t346 * t346;
            let t1209 = (t703 * t197 / t18 / t382 * t84 * t27 * t37 * v_sigma / f64x8::splat(165888.0) - t614 * t117 * t615 / f64x8::splat(3456.0) - t1150 + t1153 - t1156 + t1159) * t69 - f64x8::splat(4.0) * t1066 * t169 + f64x8::splat(12.0) * t1027 * t329 - f64x8::splat(6.0) * t568 * t346 - f64x8::splat(24.0) * t814 * t641 + f64x8::splat(24.0) * t499 * t644 - f64x8::splat(4.0) * t282 * t662 + f64x8::splat(24.0) * t754 * t1174 - f64x8::splat(36.0) * t418 * t329 * t346 + f64x8::splat(6.0) * t227 * t1180 + f64x8::splat(8.0) * t227 * t169 * t662 - t110 * (-f64x8::splat(5.0) / f64x8::splat(64.0) * t46 / t47 / t199 * t65 + f64x8::splat(15.0) / f64x8::splat(32.0) * t117 * t624 * t28 * t166 + f64x8::splat(15.0) / f64x8::splat(8.0) * t84 * t318 * t27 * t343 + f64x8::splat(81.0) * t197 * t154 * t307 * t447 - f64x8::splat(2430.0) * t613 * t903 * t1118 * t150 + t1150 - t1153 + t1156 - t1159);
            let t1213 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1209));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t1213;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
