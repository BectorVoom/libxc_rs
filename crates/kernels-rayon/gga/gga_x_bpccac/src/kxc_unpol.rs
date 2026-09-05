//! GGA_X_BPCCAC kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bpccac.c`
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
pub fn gga_x_bpccac_kxc_unpol(
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
            let t20 = ((v_sigma).sqrt());
            let t21 = f64x8::splat(M_CBRT2);
            let t24 = f64x8::splat(1.0) / t18 / v_rho;
            let t25 = t20 * t21 * t24;
            let t27 = (simd::exp(-t25 + f64x8::splat(19.0)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = f64x8::splat(1.0) - t29;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t31 * t35;
            let t37 = t21 * t21;
            let t38 = v_sigma * t37;
            let t39 = v_rho * v_rho;
            let t40 = t18 * t18;
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t38 * t42;
            let t44 = t36 * t43;
            let t46 = f64x8::splat(1.227) + f64x8::splat(0.009146457198521547) * t44;
            let t49 = f64x8::splat(2.227) - f64x8::splat(1.505529) / t46;
            let t52 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t44));
            let t55 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t52) * t31;
            let t56 = t55 * t35;
            let t59 = t31 * t31;
            let t61 = f64x8::splat(1.0) / t33 / t32;
            let t62 = t59 * t61;
            let t63 = v_sigma * v_sigma;
            let t64 = t63 * t21;
            let t65 = t39 * t39;
            let t66 = t65 * v_rho;
            let t68 = f64x8::splat(1.0) / t18 / t66;
            let t71 = f64x8::splat(1.388888888888889e-05) * t62 * t64 * t68;
            let t72 = t56 * t43 / f64x8::splat(24.0) - t71;
            let t74 = t59 / t33;
            let t75 = t74 * t20;
            let t76 = t21 * t24;
            let t79 = (simd::ln(f64x8::splat(0.6496333333333333) * t74 * t25 + ((((f64x8::splat(0.6496333333333333) * t74 * t25) * (f64x8::splat(0.6496333333333333) * t74 * t25)) + f64x8::splat(1.0)).sqrt())));
            let t80 = t76 * t79;
            let t83 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t75 * t80 + t71;
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t72 * t84 + f64x8::splat(1.0);
            let t88 = t29 * t86 + t30 * t49;
            let t92 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t88));
            let tzk0 = f64x8::splat(2.0) * t92;
            acc_zk = tzk0;
            let t94 = t17 / t40;
            let t98 = t28 * t28;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t99 * t20;
            let t101 = t100 * t21;
            let t103 = f64x8::splat(1.0) / t18 / t39;
            let t105 = t103 * t27 * t49;
            let t108 = t46 * t46;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t30 * t109 * t31;
            let t112 = t35 * v_sigma;
            let t113 = t39 * v_rho;
            let t115 = f64x8::splat(1.0) / t40 / t113;
            let t116 = t37 * t115;
            let t120 = t99 * t86;
            let t121 = t120 * t20;
            let t122 = t21 * t103;
            let t123 = t122 * t27;
            let t126 = t62 * t63;
            let t127 = t65 * t39;
            let t129 = f64x8::splat(1.0) / t18 / t127;
            let t130 = t21 * t129;
            let t131 = t130 * t52;
            let t139 = f64x8::splat(7.407407407407407e-05) * t62 * t64 * t129;
            let t140 = -f64x8::splat(0.13962962962962963) * t126 * t131 - t56 * t38 * t115 / f64x8::splat(9.0) + t139;
            let t142 = t83 * t83;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t72 * t143;
            let t145 = t122 * t79;
            let t148 = t36 * v_sigma;
            let t150 = f64x8::splat(2.532140806666667) * t44 + f64x8::splat(1.0);
            let t151 = ((t150).sqrt());
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t116 * t152;
            let t156 = -f64x8::splat(0.02182777777777778) * t75 * t145 - f64x8::splat(0.08508031222222222) * t148 * t153 - t139;
            let t158 = t140 * t84 - t144 * t156;
            let t160 = f64x8::splat(4.0) / f64x8::splat(3.0) * t101 * t105 - f64x8::splat(0.03672068415902118) * t111 * t112 * t116 - f64x8::splat(4.0) / f64x8::splat(3.0) * t121 * t123 + t29 * t158;
            let t165 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t88 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t165 + f64x8::splat(2.0) * t92;
            acc_vrho = tvrho0;
            let t168 = f64x8::splat(1.0) / t20;
            let t169 = t99 * t168;
            let t170 = t169 * t21;
            let t172 = t24 * t27 * t49;
            let t175 = t35 * t37;
            let t176 = t175 * t42;
            let t179 = t120 * t168;
            let t180 = t76 * t27;
            let t183 = t62 * t21;
            let t190 = v_sigma * t21;
            let t193 = f64x8::splat(2.777777777777778e-05) * t62 * t190 * t68;
            let t194 = f64x8::splat(0.05236111111111111) * t183 * t68 * t52 * v_sigma + t55 * t176 / f64x8::splat(24.0) - t193;
            let t196 = t74 * t168;
            let t199 = t37 * t42;
            let t200 = t199 * t152;
            let t203 = f64x8::splat(0.008185416666666667) * t196 * t80 + f64x8::splat(0.03190511708333333) * t36 * t200 + t193;
            let t205 = -t144 * t203 + t194 * t84;
            let t207 = -t170 * t172 / f64x8::splat(2.0) + f64x8::splat(0.013770256559632944) * t111 * t176 + t179 * t180 / f64x8::splat(2.0) + t29 * t205;
            let t211 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t207));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t211;
            acc_vsigma = tvsigma0;
            let t216 = t17 / t40 / v_rho;
            let t224 = f64x8::splat(1.0) / t98 / t28;
            let t226 = t224 * v_sigma * t37;
            let t228 = f64x8::splat(1.0) / t40 / t65;
            let t229 = t27 * t27;
            let t231 = t228 * t229 * t49;
            let t235 = f64x8::splat(1.0) / t18 / t113;
            let t237 = t235 * t27 * t49;
            let t240 = t99 * v_sigma;
            let t241 = t240 * t37;
            let t243 = t228 * t27 * t49;
            let t246 = t20 * v_sigma;
            let t247 = t99 * t246;
            let t248 = f64x8::splat(1.0) / t127;
            let t251 = t27 * t109 * t36;
            let t255 = f64x8::splat(1.0) / t108 / t46;
            let t257 = t30 * t255 * t59;
            let t258 = t61 * t63;
            let t259 = t65 * t113;
            let t261 = f64x8::splat(1.0) / t18 / t259;
            let t262 = t21 * t261;
            let t266 = t37 * t228;
            let t270 = t224 * t86;
            let t271 = t270 * v_sigma;
            let t272 = t266 * t229;
            let t275 = t99 * t158;
            let t276 = t275 * t20;
            let t279 = t21 * t235;
            let t280 = t279 * t27;
            let t283 = t120 * v_sigma;
            let t284 = t266 * t27;
            let t287 = t262 * t52;
            let t290 = t32 * t32;
            let t291 = f64x8::splat(1.0) / t290;
            let t292 = t63 * v_sigma;
            let t293 = t291 * t292;
            let t294 = t65 * t65;
            let t295 = t294 * t39;
            let t296 = f64x8::splat(1.0) / t295;
            let t305 = f64x8::splat(0.0004691358024691358) * t62 * t64 * t261;
            let t306 = f64x8::splat(1.2566666666666666) * t126 * t287 - f64x8::splat(18.617283950617285) * t293 * t296 * t52 + f64x8::splat(11.0) / f64x8::splat(27.0) * t56 * t38 * t228 - t305;
            let t308 = t140 * t143;
            let t312 = f64x8::splat(1.0) / t142 / t83;
            let t313 = t72 * t312;
            let t314 = t156 * t156;
            let t317 = t279 * t79;
            let t320 = t266 * t152;
            let t324 = f64x8::splat(1.0) / t151 / t150;
            let t325 = t262 * t324;
            let t328 = f64x8::splat(0.05093148148148148) * t75 * t317 + f64x8::splat(0.4254015611111111) * t148 * t320 - f64x8::splat(0.5744942144582124) * t126 * t325 + t305;
            let t330 = -t144 * t328 - f64x8::splat(2.0) * t308 * t156 + t306 * t84 + f64x8::splat(2.0) * t313 * t314;
            let t332 = -f64x8::splat(32.0) / f64x8::splat(9.0) * t226 * t231 - f64x8::splat(28.0) / f64x8::splat(9.0) * t101 * t237 + f64x8::splat(16.0) / f64x8::splat(9.0) * t241 * t243 - f64x8::splat(0.19584364884811298) * t247 * t248 * t251 - f64x8::splat(0.0035825511035830976) * t257 * t258 * t262 + f64x8::splat(0.1346425085830777) * t111 * t112 * t266 + f64x8::splat(32.0) / f64x8::splat(9.0) * t271 * t272 - f64x8::splat(8.0) / f64x8::splat(3.0) * t276 * t123 + f64x8::splat(28.0) / f64x8::splat(9.0) * t121 * t280 - f64x8::splat(16.0) / f64x8::splat(9.0) * t283 * t284 + t29 * t330;
            let t337 = ((t2).select(f64x8::splat(0.0), t6 * t216 * t88 / f64x8::splat(12.0) - t6 * t94 * t160 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t332));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t337 + f64x8::splat(4.0) * t165;
            acc_v2rho2 = tv2rho20;
            let t343 = t224 * t37;
            let t345 = t115 * t229 * t49;
            let t350 = t99 * t37;
            let t352 = t115 * t27 * t49;
            let t355 = f64x8::splat(1.0) / t66;
            let t359 = t61 * t21;
            let t364 = t175 * t115;
            let t367 = t116 * t229;
            let t370 = t275 * t168;
            let t375 = t116 * t27;
            let t378 = t99 * t205;
            let t379 = t378 * t20;
            let t386 = t294 * v_rho;
            let t387 = f64x8::splat(1.0) / t386;
            let t388 = t291 * t387;
            let t389 = t63 * t52;
            let t396 = f64x8::splat(0.00014814814814814815) * t62 * t190 * t129;
            let t397 = -f64x8::splat(0.41888888888888887) * t183 * t129 * t52 * v_sigma + f64x8::splat(6.981481481481482) * t388 * t389 - t55 * t364 / f64x8::splat(9.0) + t396;
            let t399 = t194 * t143;
            let t402 = t203 * t156;
            let t413 = -f64x8::splat(0.01091388888888889) * t196 * t145 - f64x8::splat(0.12762046833333332) * t36 * t153 + f64x8::splat(0.21543533042182963) * t183 * t129 * t324 * v_sigma - t396;
            let t415 = -t144 * t413 - t399 * t156 - t308 * t203 + f64x8::splat(2.0) * t313 * t402 + t397 * t84;
            let t417 = f64x8::splat(4.0) / f64x8::splat(3.0) * t343 * t345 + f64x8::splat(2.0) / f64x8::splat(3.0) * t170 * t105 - f64x8::splat(2.0) / f64x8::splat(3.0) * t350 * t352 + f64x8::splat(0.07344136831804236) * t100 * t355 * t251 + f64x8::splat(0.0013434566638436617) * t257 * t359 * t129 * v_sigma - f64x8::splat(0.03672068415902118) * t111 * t364 - f64x8::splat(4.0) / f64x8::splat(3.0) * t270 * t367 + t370 * t180 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t179 * t123 + f64x8::splat(2.0) / f64x8::splat(3.0) * t120 * t375 - f64x8::splat(4.0) / f64x8::splat(3.0) * t379 * t123 + t29 * t415;
            let t422 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t207 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t417));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t422 + f64x8::splat(2.0) * t211;
            acc_v2rhosigma = tv2rhosigma0;
            let t425 = f64x8::splat(1.0) / v_sigma;
            let t426 = t224 * t425;
            let t427 = t426 * t37;
            let t429 = t42 * t229 * t49;
            let t432 = f64x8::splat(1.0) / t246;
            let t433 = t99 * t432;
            let t434 = t433 * t21;
            let t437 = t99 * t425;
            let t438 = t437 * t37;
            let t440 = t42 * t27 * t49;
            let t443 = f64x8::splat(1.0) / t65;
            let t450 = t270 * t425;
            let t451 = t199 * t229;
            let t454 = t378 * t168;
            let t456 = t120 * t432;
            let t459 = t120 * t425;
            let t460 = t199 * t27;
            let t463 = f64x8::splat(1.0) / t294;
            let t464 = t291 * t463;
            let t465 = t52 * v_sigma;
            let t468 = t21 * t68;
            let t473 = f64x8::splat(2.777777777777778e-05) * t62 * t468;
            let t474 = -f64x8::splat(2.6180555555555554) * t464 * t465 + f64x8::splat(0.10472222222222222) * t62 * t468 * t52 - t473;
            let t478 = t203 * t203;
            let t481 = t74 * t432;
            let t484 = t36 * t425;
            let t487 = t468 * t324;
            let t490 = -f64x8::splat(0.004092708333333334) * t481 * t80 + f64x8::splat(0.015952558541666665) * t484 * t200 - f64x8::splat(0.08078824890818612) * t62 * t487 + t473;
            let t492 = -t144 * t490 - f64x8::splat(2.0) * t399 * t203 + f64x8::splat(2.0) * t313 * t478 + t474 * t84;
            let t494 = -t427 * t429 / f64x8::splat(2.0) + t434 * t172 / f64x8::splat(4.0) + t438 * t440 / f64x8::splat(4.0) - f64x8::splat(0.027540513119265888) * t169 * t443 * t251 - f64x8::splat(0.0005037962489413731) * t257 * t359 * t68 + t450 * t451 / f64x8::splat(2.0) + t454 * t180 - t456 * t180 / f64x8::splat(4.0) - t459 * t460 / f64x8::splat(4.0) + t29 * t492;
            let t498 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t494));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t498;
            acc_v2sigma2 = tv2sigma20;
            let t501 = t17 * t42;
            let t512 = f64x8::splat(1.0) / t18 / t294;
            let t513 = t21 * t512;
            let t517 = t294 * t113;
            let t518 = f64x8::splat(1.0) / t517;
            let t522 = t63 * t63;
            let t523 = t291 * t522;
            let t524 = t294 * t66;
            let t526 = f64x8::splat(1.0) / t40 / t524;
            let t528 = t37 * t52;
            let t529 = t36 * t528;
            let t533 = f64x8::splat(1.0) / t40 / t66;
            let t539 = f64x8::splat(0.003440329218106996) * t62 * t64 * t512;
            let t540 = -f64x8::splat(10.58082304526749) * t126 * t513 * t52 + f64x8::splat(353.7283950617284) * t293 * t518 * t52 - f64x8::splat(206.85871056241427) * t523 * t526 * t529 - f64x8::splat(154.0) / f64x8::splat(81.0) * t56 * t38 * t533 + t539;
            let t542 = t306 * t143;
            let t545 = t140 * t312;
            let t550 = t142 * t142;
            let t551 = f64x8::splat(1.0) / t550;
            let t552 = t72 * t551;
            let t553 = t314 * t156;
            let t556 = t156 * t328;
            let t560 = f64x8::splat(1.0) / t18 / t65;
            let t561 = t21 * t560;
            let t562 = t561 * t79;
            let t565 = t37 * t533;
            let t566 = t565 * t152;
            let t572 = t292 * t518;
            let t573 = t150 * t150;
            let t575 = f64x8::splat(1.0) / t151 / t573;
            let t578 = -f64x8::splat(0.1697716049382716) * t75 * t562 - f64x8::splat(2.249901589876543) * t148 * t566 + f64x8::splat(7.085428644984619) * t126 * t513 * t324 - f64x8::splat(0.7168284905723689) * t572 * t575 - t539;
            let t580 = -t144 * t578 - f64x8::splat(3.0) * t542 * t156 - f64x8::splat(3.0) * t308 * t328 + f64x8::splat(6.0) * t313 * t556 + f64x8::splat(6.0) * t545 * t314 + t540 * t84 - f64x8::splat(6.0) * t552 * t553;
            let t582 = t275 * v_sigma;
            let t585 = t565 * t27;
            let t588 = t99 * t330;
            let t589 = t588 * t20;
            let t593 = t533 * t27 * t49;
            let t596 = t224 * t158;
            let t597 = t596 * v_sigma;
            let t600 = t561 * t27;
            let t605 = t565 * t229;
            let t609 = t560 * t27 * t49;
            let t612 = f64x8::splat(1.0) / t259;
            let t617 = t533 * t229 * t49;
            let t620 = t29 * t580 - f64x8::splat(16.0) / f64x8::splat(3.0) * t582 * t284 + f64x8::splat(112.0) / f64x8::splat(9.0) * t283 * t585 - f64x8::splat(4.0) * t589 * t123 - f64x8::splat(112.0) / f64x8::splat(9.0) * t241 * t593 + f64x8::splat(32.0) / f64x8::splat(3.0) * t597 * t272 - f64x8::splat(280.0) / f64x8::splat(27.0) * t121 * t600 + f64x8::splat(28.0) / f64x8::splat(3.0) * t276 * t280 - f64x8::splat(224.0) / f64x8::splat(9.0) * t271 * t605 + f64x8::splat(280.0) / f64x8::splat(27.0) * t101 * t609 + f64x8::splat(1.7625928396330168) * t247 * t612 * t251 + f64x8::splat(224.0) / f64x8::splat(9.0) * t226 * t617;
            let t621 = t246 * t612;
            let t622 = t621 * t27;
            let t625 = t98 * t98;
            let t626 = f64x8::splat(1.0) / t625;
            let t627 = t626 * t86;
            let t628 = t229 * t27;
            let t629 = t621 * t628;
            let t632 = t621 * t229;
            let t635 = t626 * t246;
            let t636 = t612 * t628;
            let t640 = t224 * t246;
            let t641 = t612 * t229;
            let t649 = t20 * t63;
            let t650 = t99 * t649;
            let t652 = f64x8::splat(1.0) / t40 / t386;
            let t653 = t652 * t27;
            let t655 = t255 * t59;
            let t657 = t655 * t61 * t37;
            let t666 = t224 * t63;
            let t668 = t229 * t109;
            let t669 = t668 * t36;
            let t672 = t99 * t63;
            let t676 = t108 * t108;
            let t677 = f64x8::splat(1.0) / t676;
            let t678 = t30 * t677;
            let t681 = -f64x8::splat(128.0) / f64x8::splat(27.0) * t120 * t622 - f64x8::splat(256.0) / f64x8::splat(9.0) * t627 * t629 + f64x8::splat(256.0) / f64x8::splat(9.0) * t270 * t632 + f64x8::splat(256.0) / f64x8::splat(9.0) * t635 * t636 * t49 - f64x8::splat(256.0) / f64x8::splat(9.0) * t640 * t641 * t49 + f64x8::splat(128.0) / f64x8::splat(27.0) * t247 * t612 * t27 * t49 - f64x8::splat(0.01433020441433239) * t650 * t653 * t657 + f64x8::splat(0.03940806213941408) * t257 * t258 * t513 - f64x8::splat(0.6283317067210291) * t111 * t112 * t565 + f64x8::splat(0.7833745953924519) * t666 * t513 * t669 - f64x8::splat(0.39168729769622596) * t672 * t513 * t251 - f64x8::splat(3.229364321471879e-05) * t678 * t572;
            let t682 = t620 + t681;
            let t687 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t501 * t88 + t6 * t216 * t160 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t94 * t332 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t682));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t687 + f64x8::splat(6.0) * t337;
            acc_v3rho3 = tv3rho30;
            let t703 = t175 * t228;
            let t708 = t588 * t168;
            let t711 = t99 * t415;
            let t712 = t711 * t20;
            let t715 = t224 * t205;
            let t716 = t715 * v_sigma;
            let t719 = t378 * v_sigma;
            let t730 = t628 * t49;
            let t731 = t730 * t20;
            let t735 = t229 * t49;
            let t736 = t735 * t20;
            let t741 = f64x8::splat(28.0) / f64x8::splat(9.0) * t379 * t280 - f64x8::splat(4.0) / f64x8::splat(3.0) * t370 * t123 + f64x8::splat(14.0) / f64x8::splat(9.0) * t179 * t280 + f64x8::splat(0.1346425085830777) * t111 * t703 - f64x8::splat(14.0) / f64x8::splat(9.0) * t170 * t237 + t708 * t180 / f64x8::splat(2.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t712 * t123 + f64x8::splat(32.0) / f64x8::splat(9.0) * t716 * t272 - f64x8::splat(16.0) / f64x8::splat(9.0) * t719 * t284 - f64x8::splat(0.5140895782262966) * t100 * t248 * t251 - f64x8::splat(0.012091109974592954) * t257 * t359 * t261 * v_sigma - f64x8::splat(32.0) / f64x8::splat(3.0) * t626 * t248 * t731 + f64x8::splat(32.0) / f64x8::splat(3.0) * t224 * t248 * t736 + f64x8::splat(10.0) / f64x8::splat(3.0) * t350 * t243;
            let t744 = t20 * t27 * t49;
            let t756 = t248 * t20 * t27;
            let t761 = t248 * t628;
            let t762 = t761 * t20;
            let t765 = t248 * t229;
            let t766 = t765 * t20;
            let t773 = t291 * t296;
            let t776 = t294 * t65;
            let t778 = f64x8::splat(1.0) / t40 / t776;
            let t779 = t291 * t778;
            let t787 = f64x8::splat(0.0009382716049382716) * t62 * t190 * t261;
            let t788 = f64x8::splat(3.025308641975309) * t183 * t261 * t52 * v_sigma - f64x8::splat(118.68518518518519) * t773 * t389 + f64x8::splat(77.57201646090535) * t779 * t292 * t529 + f64x8::splat(11.0) / f64x8::splat(27.0) * t55 * t703 - t787;
            let t790 = t397 * t143;
            let t793 = t194 * t312;
            let t802 = t203 * t314;
            let t805 = t413 * t156;
            let t808 = t203 * t328;
            let t819 = t296 * t575;
            let t822 = f64x8::splat(0.02546574074074074) * t196 * t317 + f64x8::splat(0.5246619253703704) * t36 * t320 - f64x8::splat(2.226165081025573) * t183 * t261 * t324 * v_sigma + f64x8::splat(0.26881068396463836) * t819 * t63 + t787;
            let t824 = -t144 * t822 - f64x8::splat(2.0) * t790 * t156 - t542 * t203 - f64x8::splat(2.0) * t308 * t413 + f64x8::splat(4.0) * t313 * t805 + f64x8::splat(2.0) * t313 * t808 + f64x8::splat(2.0) * t793 * t314 - t399 * t328 + f64x8::splat(4.0) * t545 * t402 - f64x8::splat(6.0) * t552 * t802 + t788 * t84;
            let t826 = t224 * t21;
            let t827 = t261 * t229;
            let t829 = t109 * t31;
            let t830 = t829 * t112;
            let t833 = t99 * t21;
            let t839 = f64x8::splat(1.0) / t40 / t294;
            let t840 = t839 * t27;
            let t847 = -f64x8::splat(16.0) / f64x8::splat(9.0) * t99 * t248 * t744 - f64x8::splat(20.0) / f64x8::splat(3.0) * t343 * t231 + f64x8::splat(20.0) / f64x8::splat(3.0) * t270 * t272 + f64x8::splat(4.0) / f64x8::splat(3.0) * t275 * t375 - f64x8::splat(10.0) / f64x8::splat(3.0) * t120 * t284 + f64x8::splat(16.0) / f64x8::splat(9.0) * t120 * t756 - f64x8::splat(8.0) / f64x8::splat(3.0) * t596 * t367 + f64x8::splat(32.0) / f64x8::splat(3.0) * t627 * t762 - f64x8::splat(32.0) / f64x8::splat(3.0) * t270 * t766 + t29 * t824 - f64x8::splat(0.29376547327216945) * t826 * t827 * t830 + f64x8::splat(0.14688273663608473) * t833 * t261 * t27 * t830 + f64x8::splat(0.005373826655374647) * t247 * t840 * t657 + f64x8::splat(1.2110116205519546e-05) * t678 * t296 * t63;
            let t848 = t741 + t847;
            let t853 = ((t2).select(f64x8::splat(0.0), t6 * t216 * t207 / f64x8::splat(12.0) - t6 * t94 * t417 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t848));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t853 + f64x8::splat(4.0) * t422;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t872 = t168 * t355;
            let t873 = t872 * t27;
            let t876 = t872 * t628;
            let t879 = t872 * t229;
            let t886 = t626 * t168;
            let t891 = t224 * t168;
            let t900 = f64x8::splat(0.09180171039755296) * t169 * t355 * t251 - f64x8::splat(0.055081026238531776) * t833 * t129 * t251 + f64x8::splat(0.11016205247706355) * t826 * t129 * t669 + f64x8::splat(2.0) / f64x8::splat(3.0) * t169 * t355 * t27 * t49 - f64x8::splat(2.0) / f64x8::splat(3.0) * t120 * t873 - f64x8::splat(4.0) * t627 * t876 + f64x8::splat(4.0) * t270 * t879 - f64x8::splat(8.0) / f64x8::splat(3.0) * t715 * t367 + f64x8::splat(4.0) / f64x8::splat(3.0) * t378 * t375 + f64x8::splat(4.0) * t886 * t355 * t628 * t49 - f64x8::splat(4.0) * t891 * t355 * t229 * t49 - f64x8::splat(4.0) / f64x8::splat(3.0) * t454 * t123 + t456 * t123 / f64x8::splat(3.0);
            let t910 = t275 * t432;
            let t915 = t596 * t425;
            let t918 = t711 * t168;
            let t920 = t99 * t492;
            let t921 = t920 * t20;
            let t926 = t275 * t425;
            let t932 = f64x8::splat(1.0) / t40 / t517;
            let t934 = t291 * t932 * t31;
            let t936 = t35 * t63 * t528;
            let t942 = f64x8::splat(0.00014814814814814815) * t62 * t130;
            let t943 = f64x8::splat(34.907407407407405) * t388 * t465 - f64x8::splat(29.089506172839506) * t934 * t936 - f64x8::splat(0.5585185185185185) * t62 * t131 + t942;
            let t945 = t474 * t143;
            let t955 = t478 * t156;
            let t958 = t203 * t413;
            let t962 = t490 * t156;
            let t969 = t130 * t324;
            let t972 = t387 * t575;
            let t975 = f64x8::splat(0.005456944444444445) * t481 * t145 - f64x8::splat(0.021270078055555555) * t484 * t153 + f64x8::splat(0.5385883260545741) * t62 * t969 - f64x8::splat(0.10080400648673937) * t972 * v_sigma - t942;
            let t977 = -t144 * t975 - t945 * t156 - f64x8::splat(2.0) * t790 * t203 - t308 * t490 + f64x8::splat(4.0) * t313 * t958 + f64x8::splat(2.0) * t313 * t962 - f64x8::splat(2.0) * t399 * t413 + f64x8::splat(4.0) * t793 * t402 + f64x8::splat(2.0) * t545 * t478 - f64x8::splat(6.0) * t552 * t955 + t943 * t84;
            let t980 = f64x8::splat(1.0) / t40 / t259;
            let t981 = t980 * t27;
            let t988 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t450 * t367 + f64x8::splat(0.0026869133276873234) * t257 * t359 * t129 + f64x8::splat(2.0) / f64x8::splat(3.0) * t427 * t345 - t434 * t105 / f64x8::splat(3.0) - t910 * t180 / f64x8::splat(4.0) - t438 * t352 / f64x8::splat(3.0) + t915 * t451 / f64x8::splat(2.0) + t918 * t180 - f64x8::splat(4.0) / f64x8::splat(3.0) * t921 * t123 + t459 * t375 / f64x8::splat(3.0) - t926 * t460 / f64x8::splat(4.0) + t29 * t977 - f64x8::splat(0.0020151849957654924) * t100 * t981 * t657 - f64x8::splat(4.541293577069829e-06) * t678 * t387 * v_sigma;
            let t989 = t900 + t988;
            let t994 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t494 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t989));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t994 + f64x8::splat(2.0) * t498;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t997 = t378 * t432;
            let t1000 = f64x8::splat(1.0) / t649;
            let t1001 = t120 * t1000;
            let t1004 = f64x8::splat(1.0) / t63;
            let t1005 = t270 * t1004;
            let t1008 = t378 * t425;
            let t1011 = t120 * t1004;
            let t1014 = t99 * t1004;
            let t1015 = t1014 * t37;
            let t1018 = t715 * t425;
            let t1021 = t920 * t168;
            let t1025 = f64x8::splat(1.0) / t40 / t295;
            let t1027 = t291 * t1025 * t31;
            let t1028 = t175 * t465;
            let t1033 = f64x8::splat(10.908564814814815) * t1027 * t1028 - f64x8::splat(7.854166666666667) * t464 * t52;
            let t1041 = t478 * t203;
            let t1044 = t203 * t490;
            let t1047 = t74 * t1000;
            let t1050 = t36 * t1004;
            let t1053 = t62 * t425;
            let t1058 = f64x8::splat(0.0061390625) * t1047 * t80 - f64x8::splat(0.0239288378125) * t1050 * t200 - f64x8::splat(0.04039412445409306) * t1053 * t487 + f64x8::splat(0.037801502432527265) * t463 * t575;
            let t1060 = t1033 * t84 - f64x8::splat(6.0) * t552 * t1041 + f64x8::splat(6.0) * t313 * t1044 - t144 * t1058 - f64x8::splat(3.0) * t945 * t203 - f64x8::splat(3.0) * t399 * t490 + f64x8::splat(6.0) * t793 * t478;
            let t1062 = t432 * t443;
            let t1063 = t1062 * t27;
            let t1066 = t1062 * t628;
            let t1069 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t997 * t180 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1001 * t180 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1005 * t451 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1008 * t460 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1011 * t460 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1015 * t440 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1018 * t451 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1021 * t180 + t29 * t1060 + t120 * t1063 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t627 * t1066;
            let t1070 = t1062 * t229;
            let t1073 = t626 * t432;
            let t1075 = t443 * t628 * t49;
            let t1078 = t224 * t432;
            let t1080 = t443 * t229 * t49;
            let t1084 = t443 * t27 * t49;
            let t1087 = t224 * t1004;
            let t1088 = t1087 * t37;
            let t1091 = t99 * t1000;
            let t1092 = t1091 * t21;
            let t1107 = f64x8::splat(1.0) / t40 / t127;
            let t1108 = t1107 * t27;
            let t1112 = -f64x8::splat(3.0) / f64x8::splat(2.0) * t270 * t1070 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1073 * t1075 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1078 * t1080 - t433 * t1084 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t1088 * t429 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1092 * t172 + f64x8::splat(0.020655384839449415) * t433 * t443 * t251 + f64x8::splat(1.702985091401186e-06) * t678 * t463 - f64x8::splat(0.04131076967889883) * t426 * t468 * t669 + f64x8::splat(0.020655384839449415) * t437 * t468 * t251 + f64x8::splat(0.0007556943734120596) * t169 * t1108 * t657;
            let t1113 = t1069 + t1112;
            let t1117 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1113));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t1117;
            acc_v3sigma3 = tv3sigma30;
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
        ip += 8;
    }
}
