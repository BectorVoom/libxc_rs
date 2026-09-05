//! GGA_X_PW91 kxc unpol kernel — explicit SIMD (bit-exact).
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pw91_kxc_unpol(
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
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}
