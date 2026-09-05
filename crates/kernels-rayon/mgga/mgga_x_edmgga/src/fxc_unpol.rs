//! MGGA_X_EDMGGA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
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
pub fn mgga_x_edmgga_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t25 = (simd::cbrt(t24));
            let t27 = t21 * t22 * t25 / f64x8::splat(9.0);
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_tau * t30;
            let t32 = t19 * t19;
            let t34 = f64x8::splat(1.0) / t32 / v_rho;
            let t36 = v_sigma * t30;
            let t37 = v_rho * v_rho;
            let t39 = f64x8::splat(1.0) / t32 / t37;
            let t42 = v_lapl * t30;
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = t25 * t25;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = (t31 * t34 - t36 * t39 / f64x8::splat(8.0) - t42 * t34 / f64x8::splat(4.0)) * t46 * t49;
            let t51 = f64x8::splat(5.0) / f64x8::splat(9.0) * t50;
            let t52 = (-t51).simd_lt(-f64x8::splat(14205.545454545454));
            let t53 = f64x8::splat(0.39111111111111113) * t50;
            let t55 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t53);
            let t57 = ((t55).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t53));
            let t60 = t57 * t57;
            let t61 = t60 * t57;
            let t62 = f64x8::splat(1.0) / t61;
            let t65 = f64x8::splat(1.0) - t51;
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t66;
            let t69 = ((t68).sqrt());
            let t71 = ((t52).select(-f64x8::splat(1.0) / t57 / f64x8::splat(2.0) + t62 / f64x8::splat(8.0), f64x8::splat(0.704) - t53 + t69));
            let t72 = t28 * t71;
            let t73 = ((f64x8::splat(30.0)).sqrt());
            let t74 = t28 * t73;
            let t75 = ((t71).sqrt());
            let t76 = t28 * t28;
            let t81 = f64x8::splat(0.6018478308354863) * t76 - f64x8::splat(0.0206514);
            let t82 = t71 - f64x8::splat(1.0);
            let t86 = (simd::ln(f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82 + ((((f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82) * (f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82)) + f64x8::splat(1.0)).sqrt())));
            let t90 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t74 * t75 * t86;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = t72 * t91 + t27;
            let t97 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t93));
            let tzk0 = f64x8::splat(2.0) * t97;
            acc_zk = tzk0;
            let t99 = t18 / t32;
            let t103 = f64x8::splat(1.0) / t60;
            let t106 = t37 * v_rho;
            let t108 = f64x8::splat(1.0) / t32 / t106;
            let t113 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t31 * t39 + t36 * t108 / f64x8::splat(3.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t42 * t39;
            let t115 = t113 * t46 * t49;
            let t116 = f64x8::splat(0.39111111111111113) * t115;
            let t117 = ((t55).select(f64x8::splat(0.0), -t116));
            let t120 = t60 * t60;
            let t121 = f64x8::splat(1.0) / t120;
            let t125 = f64x8::splat(1.0) / t69;
            let t126 = t125 * t65;
            let t130 = ((t52).select(t103 * t117 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t121 * t117, -t116 - f64x8::splat(0.2753422222222222) * t126 * t115));
            let t131 = t28 * t130;
            let t133 = t90 * t90;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = f64x8::splat(1.0) / t75;
            let t136 = t135 * t86;
            let t140 = f64x8::splat(1.0) / t76;
            let t141 = t140 * t75;
            let t142 = t81 * t130;
            let t143 = t76 * t76;
            let t144 = t143 * t76;
            let t146 = t81 * t81;
            let t148 = t82 * t82;
            let t151 = f64x8::splat(4.587117884468566) / t144 * t146 * t148 + f64x8::splat(1.0);
            let t152 = ((t151).sqrt());
            let t153 = f64x8::splat(1.0) / t152;
            let t157 = f64x8::splat(0.07081947889031463) * t74 * t136 * t130 + f64x8::splat(1.661549562472956) * t141 * t142 * t153;
            let t158 = t134 * t157;
            let t160 = t131 * t91 - t72 * t158;
            let t165 = ((t3).select(f64x8::splat(0.0), -t7 * t99 * t93 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t160));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t165 + f64x8::splat(2.0) * t97;
            acc_vrho = tvrho0;
            let t168 = t30 * t39;
            let t169 = t46 * t49;
            let t170 = t168 * t169;
            let t171 = f64x8::splat(0.04888888888888889) * t170;
            let t172 = ((t55).select(f64x8::splat(0.0), t171));
            let t175 = t121 * t172;
            let t178 = t126 * t30;
            let t180 = t39 * t46 * t49;
            let t181 = t178 * t180;
            let t184 = ((t52).select(t103 * t172 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t175, t171 + f64x8::splat(0.034417777777777776) * t181));
            let t185 = t28 * t184;
            let t194 = f64x8::splat(0.07081947889031463) * t74 * t136 * t184 + f64x8::splat(1.661549562472956) * t141 * t81 * t184 * t153;
            let t195 = t134 * t194;
            let t197 = t185 * t91 - t72 * t195;
            let t201 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t197));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t201;
            acc_vsigma = tvsigma0;
            let t203 = t30 * t34;
            let t204 = t203 * t169;
            let t205 = f64x8::splat(0.09777777777777778) * t204;
            let t206 = ((t55).select(f64x8::splat(0.0), t205));
            let t209 = t121 * t206;
            let t213 = t34 * t46 * t49;
            let t214 = t178 * t213;
            let t217 = ((t52).select(t103 * t206 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t209, t205 + f64x8::splat(0.06883555555555555) * t214));
            let t218 = t28 * t217;
            let t227 = f64x8::splat(0.07081947889031463) * t74 * t136 * t217 + f64x8::splat(1.661549562472956) * t141 * t81 * t217 * t153;
            let t228 = t134 * t227;
            let t230 = t218 * t91 - t72 * t228;
            let t234 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t230));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t234;
            acc_vlapl = tvlapl0;
            let t236 = f64x8::splat(0.39111111111111113) * t204;
            let t237 = ((t55).select(f64x8::splat(0.0), -t236));
            let t240 = t121 * t237;
            let t245 = ((t52).select(t103 * t237 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t240, -t236 - f64x8::splat(0.2753422222222222) * t214));
            let t246 = t28 * t245;
            let t255 = f64x8::splat(0.07081947889031463) * t74 * t136 * t245 + f64x8::splat(1.661549562472956) * t141 * t81 * t245 * t153;
            let t256 = t134 * t255;
            let t258 = t246 * t91 - t72 * t256;
            let t262 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t258));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t262;
            acc_vtau = tvtau0;
            let t265 = t18 * t34;
            let t272 = t117 * t117;
            let t276 = t37 * t37;
            let t278 = f64x8::splat(1.0) / t32 / t276;
            let t283 = f64x8::splat(40.0) / f64x8::splat(9.0) * t31 * t108 - f64x8::splat(11.0) / f64x8::splat(9.0) * t36 * t278 - f64x8::splat(10.0) / f64x8::splat(9.0) * t42 * t108;
            let t285 = t283 * t46 * t49;
            let t286 = f64x8::splat(0.39111111111111113) * t285;
            let t287 = ((t55).select(f64x8::splat(0.0), -t286));
            let t291 = f64x8::splat(1.0) / t120 / t57;
            let t298 = f64x8::splat(1.0) / t69 / t68;
            let t299 = t298 * t66;
            let t300 = t113 * t113;
            let t301 = t46 * t46;
            let t304 = f64x8::splat(1.0) / t25 / t24;
            let t309 = t301 * t304;
            let t315 = ((t52).select(-t62 * t272 + t103 * t287 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t291 * t272 - f64x8::splat(3.0) / f64x8::splat(8.0) * t121 * t287, -t286 - f64x8::splat(0.0758133393382716) * t299 * t300 * t301 * t304 + f64x8::splat(0.1529679012345679) * t125 * t300 * t309 - f64x8::splat(0.2753422222222222) * t126 * t285));
            let t316 = t28 * t315;
            let t321 = f64x8::splat(1.0) / t133 / t90;
            let t322 = t157 * t157;
            let t323 = t321 * t322;
            let t327 = f64x8::splat(1.0) / t75 / t71;
            let t328 = t327 * t86;
            let t329 = t130 * t130;
            let t333 = t140 * t135;
            let t345 = t143 * t143;
            let t346 = f64x8::splat(1.0) / t345;
            let t347 = t346 * t75;
            let t348 = t146 * t81;
            let t349 = t347 * t348;
            let t351 = f64x8::splat(1.0) / t152 / t151;
            let t352 = t329 * t351;
            let t356 = -f64x8::splat(0.035409739445157316) * t74 * t328 * t329 + f64x8::splat(1.661549562472956) * t333 * t81 * t329 * t153 + f64x8::splat(0.07081947889031463) * t74 * t136 * t315 + f64x8::splat(1.661549562472956) * t141 * t81 * t315 * t153 - f64x8::splat(7.621723713950617) * t349 * t352 * t82;
            let t357 = t134 * t356;
            let t359 = -f64x8::splat(2.0) * t131 * t158 + t316 * t91 + f64x8::splat(2.0) * t72 * t323 - t72 * t357;
            let t364 = ((t3).select(f64x8::splat(0.0), t7 * t265 * t93 / f64x8::splat(12.0) - t7 * t99 * t160 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t359));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t364 + f64x8::splat(4.0) * t165;
            acc_v2rho2 = tv2rho20;
            let t370 = t62 * t172;
            let t372 = t30 * t108;
            let t373 = t372 * t169;
            let t374 = f64x8::splat(0.13037037037037036) * t373;
            let t375 = ((t55).select(f64x8::splat(0.0), -t374));
            let t378 = t291 * t172;
            let t381 = t121 * t375;
            let t384 = t299 * t30;
            let t385 = t39 * t301;
            let t386 = t304 * t113;
            let t388 = t384 * t385 * t386;
            let t390 = t125 * t113;
            let t391 = t390 * t301;
            let t392 = t304 * t30;
            let t393 = t392 * t39;
            let t394 = t391 * t393;
            let t398 = t178 * t108 * t46 * t49;
            let t401 = ((t52).select(-t370 * t117 + t103 * t375 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t378 * t117 - f64x8::splat(3.0) / f64x8::splat(8.0) * t381, -t374 + f64x8::splat(0.00947666741728395) * t388 - f64x8::splat(0.01912098765432099) * t394 - f64x8::splat(0.09178074074074075) * t398));
            let t402 = t28 * t401;
            let t406 = t321 * t194;
            let t407 = t406 * t157;
            let t410 = t74 * t327;
            let t411 = t86 * t184;
            let t415 = t333 * t81;
            let t416 = t130 * t153;
            let t427 = t184 * t351;
            let t428 = t82 * t130;
            let t429 = t427 * t428;
            let t432 = -f64x8::splat(0.035409739445157316) * t410 * t411 * t130 + f64x8::splat(1.661549562472956) * t415 * t416 * t184 + f64x8::splat(0.07081947889031463) * t74 * t136 * t401 + f64x8::splat(1.661549562472956) * t141 * t81 * t401 * t153 - f64x8::splat(7.621723713950617) * t349 * t429;
            let t433 = t134 * t432;
            let t435 = -t131 * t195 - t185 * t158 + t402 * t91 + f64x8::splat(2.0) * t72 * t407 - t72 * t433;
            let t440 = ((t3).select(f64x8::splat(0.0), -t7 * t99 * t197 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t435));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t440 + f64x8::splat(2.0) * t201;
            acc_v2rhosigma = tv2rhosigma0;
            let t446 = t62 * t206;
            let t448 = f64x8::splat(0.16296296296296298) * t170;
            let t449 = ((t55).select(f64x8::splat(0.0), -t448));
            let t452 = t291 * t206;
            let t455 = t121 * t449;
            let t458 = t34 * t301;
            let t460 = t384 * t458 * t386;
            let t462 = t392 * t34;
            let t463 = t391 * t462;
            let t467 = ((t52).select(-t446 * t117 + t103 * t449 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t452 * t117 - f64x8::splat(3.0) / f64x8::splat(8.0) * t455, -t448 + f64x8::splat(0.0189533348345679) * t460 - f64x8::splat(0.03824197530864198) * t463 - f64x8::splat(0.11472592592592593) * t181));
            let t468 = t28 * t467;
            let t472 = t321 * t227;
            let t473 = t472 * t157;
            let t476 = t86 * t217;
            let t490 = t217 * t351;
            let t491 = t490 * t428;
            let t494 = -f64x8::splat(0.035409739445157316) * t410 * t476 * t130 + f64x8::splat(1.661549562472956) * t415 * t416 * t217 + f64x8::splat(0.07081947889031463) * t74 * t136 * t467 + f64x8::splat(1.661549562472956) * t141 * t81 * t467 * t153 - f64x8::splat(7.621723713950617) * t349 * t491;
            let t495 = t134 * t494;
            let t497 = -t131 * t228 - t218 * t158 + t468 * t91 + f64x8::splat(2.0) * t72 * t473 - t72 * t495;
            let t502 = ((t3).select(f64x8::splat(0.0), -t7 * t99 * t230 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t497));
            let tv2rholapl0 = f64x8::splat(2.0) * v_rho * t502 + f64x8::splat(2.0) * t234;
            acc_v2rholapl = tv2rholapl0;
            let t508 = t62 * t237;
            let t510 = f64x8::splat(0.6518518518518519) * t170;
            let t511 = ((t55).select(f64x8::splat(0.0), t510));
            let t514 = t291 * t237;
            let t517 = t121 * t511;
            let t524 = ((t52).select(-t508 * t117 + t103 * t511 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t514 * t117 - f64x8::splat(3.0) / f64x8::splat(8.0) * t517, t510 - f64x8::splat(0.0758133393382716) * t460 + f64x8::splat(0.1529679012345679) * t463 + f64x8::splat(0.4589037037037037) * t181));
            let t525 = t28 * t524;
            let t529 = t321 * t255;
            let t530 = t529 * t157;
            let t533 = t86 * t245;
            let t547 = t245 * t351;
            let t548 = t547 * t428;
            let t551 = -f64x8::splat(0.035409739445157316) * t410 * t533 * t130 + f64x8::splat(1.661549562472956) * t415 * t416 * t245 + f64x8::splat(0.07081947889031463) * t74 * t136 * t524 + f64x8::splat(1.661549562472956) * t141 * t81 * t524 * t153 - f64x8::splat(7.621723713950617) * t349 * t548;
            let t552 = t134 * t551;
            let t554 = -t131 * t256 - t246 * t158 + t525 * t91 + f64x8::splat(2.0) * t72 * t530 - t72 * t552;
            let t559 = ((t3).select(f64x8::splat(0.0), -t7 * t99 * t258 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t554));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t559 + f64x8::splat(2.0) * t262;
            acc_v2rhotau = tv2rhotau0;
            let t562 = t172 * t172;
            let t564 = ((t55).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t566 = t103 * t564 / f64x8::splat(2.0);
            let t567 = t291 * t562;
            let t569 = t121 * t564;
            let t570 = f64x8::splat(3.0) / f64x8::splat(8.0) * t569;
            let t572 = t299 * t29;
            let t573 = t276 * v_rho;
            let t575 = f64x8::splat(1.0) / t19 / t573;
            let t577 = t575 * t301 * t304;
            let t578 = t572 * t577;
            let t580 = t125 * t29;
            let t581 = t580 * t577;
            let t584 = ((t52).select(-t62 * t562 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t567 - t570, -f64x8::splat(0.0023691668543209875) * t578 + f64x8::splat(0.004780246913580247) * t581));
            let t585 = t28 * t584;
            let t589 = t194 * t194;
            let t590 = t321 * t589;
            let t593 = t184 * t184;
            let t608 = t593 * t351;
            let t612 = -f64x8::splat(0.035409739445157316) * t74 * t328 * t593 + f64x8::splat(1.661549562472956) * t333 * t81 * t593 * t153 + f64x8::splat(0.07081947889031463) * t74 * t136 * t584 + f64x8::splat(1.661549562472956) * t141 * t81 * t584 * t153 - f64x8::splat(7.621723713950617) * t349 * t608 * t82;
            let t613 = t134 * t612;
            let t615 = -f64x8::splat(2.0) * t185 * t195 + t585 * t91 + f64x8::splat(2.0) * t72 * t590 - t72 * t613;
            let t619 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t615));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t619;
            acc_v2sigma2 = tv2sigma20;
            let t626 = f64x8::splat(1.0) / t19 / t276;
            let t628 = t626 * t301 * t304;
            let t629 = t572 * t628;
            let t631 = t580 * t628;
            let t634 = ((t52).select(-t446 * t172 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t452 * t172 - t570, -f64x8::splat(0.004738333708641975) * t629 + f64x8::splat(0.009560493827160494) * t631));
            let t635 = t28 * t634;
            let t639 = t472 * t194;
            let t645 = t184 * t153;
            let t656 = t82 * t184;
            let t657 = t490 * t656;
            let t660 = -f64x8::splat(0.035409739445157316) * t410 * t476 * t184 + f64x8::splat(1.661549562472956) * t415 * t645 * t217 + f64x8::splat(0.07081947889031463) * t74 * t136 * t634 + f64x8::splat(1.661549562472956) * t141 * t81 * t634 * t153 - f64x8::splat(7.621723713950617) * t349 * t657;
            let t661 = t134 * t660;
            let t663 = -t185 * t228 - t218 * t195 + t635 * t91 + f64x8::splat(2.0) * t72 * t639 - t72 * t661;
            let t667 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t663));
            let tv2sigmalapl0 = f64x8::splat(2.0) * v_rho * t667;
            acc_v2sigmalapl = tv2sigmalapl0;
            let t676 = ((t52).select(-t508 * t172 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t514 * t172 - t570, f64x8::splat(0.0189533348345679) * t629 - f64x8::splat(0.03824197530864198) * t631));
            let t677 = t28 * t676;
            let t681 = t529 * t194;
            let t697 = t547 * t656;
            let t700 = -f64x8::splat(0.035409739445157316) * t410 * t533 * t184 + f64x8::splat(1.661549562472956) * t415 * t645 * t245 + f64x8::splat(0.07081947889031463) * t74 * t136 * t676 + f64x8::splat(1.661549562472956) * t141 * t81 * t676 * t153 - f64x8::splat(7.621723713950617) * t349 * t697;
            let t701 = t134 * t700;
            let t703 = -t185 * t256 - t246 * t195 + t677 * t91 + f64x8::splat(2.0) * t72 * t681 - t72 * t701;
            let t707 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t703));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t707;
            acc_v2sigmatau = tv2sigmatau0;
            let t709 = t206 * t206;
            let t711 = t291 * t709;
            let t715 = f64x8::splat(1.0) / t19 / t106;
            let t717 = t715 * t301 * t304;
            let t718 = t572 * t717;
            let t720 = t580 * t717;
            let t723 = ((t52).select(-t62 * t709 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t711 - t570, -f64x8::splat(0.00947666741728395) * t718 + f64x8::splat(0.01912098765432099) * t720));
            let t724 = t28 * t723;
            let t728 = t227 * t227;
            let t729 = t321 * t728;
            let t732 = t217 * t217;
            let t747 = t732 * t351;
            let t751 = -f64x8::splat(0.035409739445157316) * t74 * t328 * t732 + f64x8::splat(1.661549562472956) * t333 * t81 * t732 * t153 + f64x8::splat(0.07081947889031463) * t74 * t136 * t723 + f64x8::splat(1.661549562472956) * t141 * t81 * t723 * t153 - f64x8::splat(7.621723713950617) * t349 * t747 * t82;
            let t752 = t134 * t751;
            let t754 = -f64x8::splat(2.0) * t218 * t228 + f64x8::splat(2.0) * t72 * t729 - t72 * t752 + t724 * t91;
            let t758 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t754));
            let tv2lapl20 = f64x8::splat(2.0) * v_rho * t758;
            acc_v2lapl2 = tv2lapl20;
            let t767 = ((t52).select(-t508 * t206 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t514 * t206 - t570, f64x8::splat(0.0379066696691358) * t718 - f64x8::splat(0.07648395061728396) * t720));
            let t768 = t28 * t767;
            let t772 = t529 * t227;
            let t778 = t217 * t153;
            let t789 = t82 * t217;
            let t790 = t547 * t789;
            let t793 = -f64x8::splat(0.035409739445157316) * t410 * t533 * t217 + f64x8::splat(1.661549562472956) * t415 * t778 * t245 + f64x8::splat(0.07081947889031463) * t74 * t136 * t767 + f64x8::splat(1.661549562472956) * t141 * t81 * t767 * t153 - f64x8::splat(7.621723713950617) * t349 * t790;
            let t794 = t134 * t793;
            let t796 = -t218 * t256 - t246 * t228 + f64x8::splat(2.0) * t72 * t772 - t72 * t794 + t768 * t91;
            let t800 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t796));
            let tv2lapltau0 = f64x8::splat(2.0) * v_rho * t800;
            acc_v2lapltau = tv2lapltau0;
            let t802 = t237 * t237;
            let t804 = t291 * t802;
            let t810 = ((t52).select(-t62 * t802 + t566 + f64x8::splat(3.0) / f64x8::splat(2.0) * t804 - t570, -f64x8::splat(0.1516266786765432) * t718 + f64x8::splat(0.3059358024691358) * t720));
            let t811 = t28 * t810;
            let t815 = t255 * t255;
            let t816 = t321 * t815;
            let t819 = t245 * t245;
            let t834 = t819 * t351;
            let t838 = -f64x8::splat(0.035409739445157316) * t74 * t328 * t819 + f64x8::splat(1.661549562472956) * t333 * t81 * t819 * t153 + f64x8::splat(0.07081947889031463) * t74 * t136 * t810 + f64x8::splat(1.661549562472956) * t141 * t81 * t810 * t153 - f64x8::splat(7.621723713950617) * t349 * t834 * t82;
            let t839 = t134 * t838;
            let t841 = -f64x8::splat(2.0) * t246 * t256 + f64x8::splat(2.0) * t72 * t816 - t72 * t839 + t811 * t91;
            let t845 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t841));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t845;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
