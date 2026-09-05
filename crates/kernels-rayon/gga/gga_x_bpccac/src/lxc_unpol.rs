//! GGA_X_BPCCAC lxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_bpccac_lxc_unpol(
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
            let t1134 = f64x8::splat(1.0) / t18 / t386;
            let t1135 = t21 * t1134;
            let t1139 = t37 * t1107;
            let t1145 = t30 / t676 / t46;
            let t1147 = t294 * t127;
            let t1149 = f64x8::splat(1.0) / t40 / t1147;
            let t1155 = f64x8::splat(1.0) / t776;
            let t1158 = t229 * t255 * t62;
            let t1164 = t27 * t255 * t62;
            let t1170 = t626 * t158;
            let t1181 = t246 * t463;
            let t1188 = -f64x8::splat(0.38970194782309475) * t257 * t258 * t1135 + f64x8::splat(3.5605463380858318) * t111 * t112 * t1139 - f64x8::splat(3.150639204776012e-06) * t1145 * t522 * t1149 * t31 * t175 + f64x8::splat(0.15285551375287884) * t224 * t292 * t1155 * t1158 - f64x8::splat(0.07642775687643942) * t99 * t292 * t1155 * t1164 - f64x8::splat(14.775315285318747) * t247 * t463 * t251 - f64x8::splat(1024.0) / f64x8::splat(9.0) * t1170 * t629 + f64x8::splat(3584.0) / f64x8::splat(9.0) * t640 * t463 * t229 * t49 - f64x8::splat(1792.0) / f64x8::splat(27.0) * t247 * t463 * t27 * t49 - f64x8::splat(3584.0) / f64x8::splat(9.0) * t270 * t1181 * t229 + f64x8::splat(1792.0) / f64x8::splat(27.0) * t120 * t1181 * t27;
            let t1211 = t294 * t294;
            let t1216 = t21 * t52;
            let t1217 = t62 * t1216;
            let t1225 = f64x8::splat(0.028669410150891632) * t62 * t64 * t1134;
            let t1228 = t540 * t143;
            let t1231 = t306 * t312;
            let t1236 = t140 * t551;
            let t1245 = t72 / t550 / t83;
            let t1246 = t314 * t314;
            let t1252 = t328 * t328;
            let t1267 = t292 * t1155;
            let t1273 = f64x8::splat(1.0) / t151 / t573 / t150;
            let t1275 = t36 * t37;
            let t1280 = (f64x8::splat(94.54477366255144) * t126 * t1135 * t52 - f64x8::splat(5301.788751714677) * t293 * t1155 * t52 + f64x8::splat(6757.384545038866) * t523 * t1149 * t529 - f64x8::splat(4596.860234720317) * t291 * t522 * v_sigma / t18 / t1211 / v_rho * t1217 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t56 * t38 * t1107 - t1225) * t84 - f64x8::splat(4.0) * t1228 * t156 + f64x8::splat(12.0) * t1231 * t314 - f64x8::splat(6.0) * t542 * t328 - f64x8::splat(24.0) * t1236 * t553 + f64x8::splat(24.0) * t545 * t556 - f64x8::splat(4.0) * t308 * t578 + f64x8::splat(24.0) * t1245 * t1246 - f64x8::splat(36.0) * t552 * t314 * t328 + f64x8::splat(6.0) * t313 * t1252 + f64x8::splat(8.0) * t313 * t156 * t578 - t144 * (f64x8::splat(0.7356769547325103) * t75 * t468 * t79 + f64x8::splat(13.631756691604938) * t148 * t1139 * t152 - f64x8::splat(74.23741904610011) * t126 * t1135 * t324 + f64x8::splat(16.725998113355274) * t1267 * t575 - f64x8::splat(12.100737815730449) * t522 * t1149 * t1273 * t1275 + t1225);
            let t1283 = t650 * t1025 * t27;
            let t1284 = t829 * t175;
            let t1287 = t1134 * t229;
            let t1290 = t829 * t35 * t21;
            let t1308 = f64x8::splat(1024.0) / f64x8::splat(9.0) * t596 * t632 - f64x8::splat(512.0) / f64x8::splat(27.0) * t275 * t622 + f64x8::splat(3584.0) / f64x8::splat(9.0) * t627 * t1181 * t628 - f64x8::splat(3584.0) / f64x8::splat(9.0) * t635 * t463 * t628 * t49 + t29 * t1280 - f64x8::splat(0.6963329736821795) * t1283 * t1284 - f64x8::splat(13.056243256540865) * t666 * t1287 * t1290 + f64x8::splat(0.2770506186770929) * t1283 * t657 - f64x8::splat(4.1779978420930775) * t626 * t649 * t1025 * t628 * t1284 + f64x8::splat(4.1779978420930775) * t224 * t649 * t1025 * t229 * t1284 + f64x8::splat(6.528121628270433) * t672 * t1135 * t251;
            let t1315 = t99 * t580;
            let t1320 = f64x8::splat(1.0) / t625 / t28;
            let t1323 = t229 * t229;
            let t1325 = t1323 * t49 * t21;
            let t1330 = t730 * t21;
            let t1334 = t735 * t21;
            let t1339 = t21 * t27 * t49;
            let t1342 = t20 * t292;
            let t1345 = f64x8::splat(1.0) / t18 / t524;
            let t1348 = t27 * t677 * t21;
            let t1359 = t1320 * t86;
            let t1365 = f64x8::splat(0.0007104601507238133) * t678 * t1267 - f64x8::splat(32.0) / f64x8::splat(3.0) * t588 * v_sigma * t284 - f64x8::splat(16.0) / f64x8::splat(3.0) * t1315 * t20 * t123 - f64x8::splat(4096.0) / f64x8::splat(27.0) * t1320 * t63 * t1134 * t1325 + f64x8::splat(2048.0) / f64x8::splat(9.0) * t626 * t63 * t1134 * t1330 - f64x8::splat(7168.0) / f64x8::splat(81.0) * t666 * t1134 * t1334 + f64x8::splat(512.0) / f64x8::splat(81.0) * t672 * t1134 * t1339 - f64x8::splat(0.00017223276381183352) * t99 * t1342 * t1345 * t1348 + f64x8::splat(7168.0) / f64x8::splat(81.0) * t270 * t63 * t1287 * t21 - f64x8::splat(512.0) / f64x8::splat(81.0) * t120 * t63 * t1135 * t27 + f64x8::splat(4096.0) / f64x8::splat(27.0) * t1359 * t63 * t1134 * t1323 * t21;
            let t1371 = t1107 * t229;
            let t1380 = t224 * t330;
            let t1390 = t468 * t27;
            let t1403 = -f64x8::splat(2048.0) / f64x8::splat(9.0) * t627 * t63 * t1134 * t628 * t21 - f64x8::splat(13664.0) / f64x8::splat(81.0) * t226 * t1371 * t49 - f64x8::splat(1120.0) / f64x8::splat(27.0) * t276 * t600 + f64x8::splat(13664.0) / f64x8::splat(81.0) * t271 * t1139 * t229 + f64x8::splat(64.0) / f64x8::splat(3.0) * t1380 * v_sigma * t272 - f64x8::splat(3640.0) / f64x8::splat(81.0) * t101 * t68 * t27 * t49 - f64x8::splat(896.0) / f64x8::splat(9.0) * t597 * t605 + f64x8::splat(3640.0) / f64x8::splat(81.0) * t121 * t1390 + f64x8::splat(56.0) / f64x8::splat(3.0) * t589 * t280 + f64x8::splat(6832.0) / f64x8::splat(81.0) * t241 * t1108 * t49 - f64x8::splat(6832.0) / f64x8::splat(81.0) * t283 * t1139 * t27 + f64x8::splat(448.0) / f64x8::splat(9.0) * t582 * t585;
            let t1410 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t115 * t88 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t501 * t160 + t6 * t216 * t332 / f64x8::splat(2.0) - t6 * t94 * t682 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1188 + t1308 + t1365 + t1403)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t1410 + f64x8::splat(8.0) * t687;
            acc_v4rho4 = tv4rho40;
            let t1440 = f64x8::splat(0.006880658436213992) * t62 * t190 * t512;
            let t1472 = t175 * t533;
            let t1482 = t788 * t143;
            let t1497 = t397 * t312;
            let t1500 = t194 * t551;
            let t1511 = f64x8::splat(6.0) * t313 * t822 * t156 + f64x8::splat(2.0) * t313 * t203 * t578 + f64x8::splat(6.0) * t313 * t413 * t328 - f64x8::splat(18.0) * t552 * t413 * t314 + f64x8::splat(6.0) * t1231 * t402 - f64x8::splat(3.0) * t1482 * t156 + f64x8::splat(6.0) * t1497 * t314 - f64x8::splat(6.0) * t1500 * t553 + f64x8::splat(12.0) * t545 * t805 + f64x8::splat(6.0) * t545 * t808 + f64x8::splat(6.0) * t793 * t556;
            let t1517 = t247 * t653;
            let t1524 = t36 * t246 * t37;
            let t1552 = t99 * t824;
            let t1561 = t29 * (-t144 * (-f64x8::splat(0.0848858024691358) * t196 * t562 - f64x8::splat(2.5807694707407407) * t36 * t566 + f64x8::splat(19.867924916679844) * t183 * t512 * t324 * v_sigma - f64x8::splat(5.465817240614313) * t518 * t575 * t63 + f64x8::splat(4.537776680898919) * t526 * t1273 * t292 * t1275 - t1440) - f64x8::splat(3.0) * t542 * t413 - f64x8::splat(3.0) * t308 * t822 + f64x8::splat(24.0) * t1245 * t203 * t553 - f64x8::splat(18.0) * t552 * t402 * t328 - f64x8::splat(18.0) * t1236 * t802 + (-f64x8::splat(23.550864197530863) * t183 * t512 * t52 * v_sigma + f64x8::splat(1590.2263374485597) * t291 * t518 * t389 - f64x8::splat(2301.3031550068586) * t291 * t526 * t292 * t529 + f64x8::splat(1723.822588020119) * t291 / t18 / t1211 * t522 * t1217 - f64x8::splat(154.0) / f64x8::splat(81.0) * t55 * t1472 + t1440) * t84 - f64x8::splat(3.0) * t790 * t328 - t399 * t578 - t1228 * t203 + t1511) - f64x8::splat(2.0073974006931583) * t240 * t513 * t251 - f64x8::splat(0.0877725020377859) * t1517 * t657 + f64x8::splat(1.5667491907849038) * t626 * t652 * t628 * t109 * t1524 - f64x8::splat(1.5667491907849038) * t224 * t652 * t668 * t1524 + f64x8::splat(0.26112486513081734) * t1517 * t1284 + f64x8::splat(4.014794801386317) * t826 * t512 * t229 * t830 - f64x8::splat(0.00023009220790487137) * t678 * t518 * t63 - f64x8::splat(896.0) / f64x8::splat(27.0) * t270 * t512 * v_sigma * t229 * t21 + f64x8::splat(64.0) / f64x8::splat(27.0) * t120 * t512 * t190 * t27 - f64x8::splat(16.0) / f64x8::splat(3.0) * t711 * v_sigma * t284 - f64x8::splat(4.0) * t1552 * t20 * t123 - f64x8::splat(512.0) / f64x8::splat(9.0) * t1359 * t512 * t1323 * v_sigma * t21;
            let t1567 = t224 * t415;
            let t1587 = t49 * v_sigma * t21;
            let t1609 = f64x8::splat(256.0) / f64x8::splat(3.0) * t627 * t512 * t628 * v_sigma * t21 + f64x8::splat(32.0) / f64x8::splat(3.0) * t1567 * v_sigma * t272 + f64x8::splat(112.0) / f64x8::splat(9.0) * t719 * t585 + f64x8::splat(28.0) / f64x8::splat(3.0) * t712 * t280 - f64x8::splat(224.0) / f64x8::splat(9.0) * t716 * t605 - f64x8::splat(2.0) * t708 * t123 + t1315 * t168 * t180 / f64x8::splat(2.0) - f64x8::splat(280.0) / f64x8::splat(27.0) * t379 * t600 - f64x8::splat(256.0) / f64x8::splat(3.0) * t626 * t512 * t628 * t1587 + f64x8::splat(896.0) / f64x8::splat(27.0) * t224 * t512 * t229 * t1587 - f64x8::splat(64.0) / f64x8::splat(27.0) * t99 * t512 * v_sigma * t1339 + f64x8::splat(6.458728642943758e-05) * t650 / t18 / t776 * t1348 + f64x8::splat(512.0) / f64x8::splat(9.0) * t1320 * t512 * t1323 * t1587 - f64x8::splat(140.0) / f64x8::splat(27.0) * t179 * t600;
            let t1638 = t626 * t205;
            let t1643 = -f64x8::splat(0.6283317067210291) * t111 * t1472 + f64x8::splat(140.0) / f64x8::splat(27.0) * t170 * t609 + f64x8::splat(14.0) / f64x8::splat(3.0) * t370 * t280 + f64x8::splat(2.0) * t588 * t375 + f64x8::splat(32.0) * t1170 * t762 - f64x8::splat(32.0) * t596 * t766 + f64x8::splat(16.0) / f64x8::splat(3.0) * t275 * t756 + f64x8::splat(352.0) / f64x8::splat(3.0) * t270 * t641 * t20 - f64x8::splat(176.0) / f64x8::splat(9.0) * t120 * t612 * t20 * t27 - f64x8::splat(4.0) * t1380 * t367 - f64x8::splat(10.0) * t275 * t284 + f64x8::splat(476.0) / f64x8::splat(27.0) * t120 * t585 - f64x8::splat(256.0) / f64x8::splat(9.0) * t1638 * t629 + f64x8::splat(256.0) / f64x8::splat(9.0) * t715 * t632;
            let t1653 = t655 * t258;
            let t1687 = -f64x8::splat(128.0) / f64x8::splat(27.0) * t378 * t622 - f64x8::splat(352.0) / f64x8::splat(3.0) * t627 * t636 * t20 + f64x8::splat(20.0) * t596 * t272 - f64x8::splat(0.05732081765732956) * t224 * t518 * t229 * t1653 + f64x8::splat(0.02866040882866478) * t99 * t518 * t27 * t1653 + f64x8::splat(1.1814897017910045e-06) * t1145 * t526 * t292 * t31 * t175 + f64x8::splat(3.557826287407386) * t100 * t612 * t251 + f64x8::splat(0.10180416052681969) * t257 * t359 * t512 * v_sigma + f64x8::splat(176.0) / f64x8::splat(9.0) * t99 * t612 * t744 - f64x8::splat(476.0) / f64x8::splat(27.0) * t350 * t593 + f64x8::splat(352.0) / f64x8::splat(3.0) * t626 * t612 * t731 - f64x8::splat(352.0) / f64x8::splat(3.0) * t224 * t612 * t736 + f64x8::splat(952.0) / f64x8::splat(27.0) * t343 * t617 - f64x8::splat(952.0) / f64x8::splat(27.0) * t270 * t605;
            let t1694 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t501 * t207 + t6 * t216 * t417 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t94 * t848 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1561 + t1609 + t1643 + t1687)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t1694 + f64x8::splat(6.0) * t853;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1715 = t829 * t35 * t20;
            let t1730 = t99 * t977;
            let t1745 = t224 * t492;
            let t1749 = f64x8::splat(28.0) / f64x8::splat(9.0) * t921 * t280 - f64x8::splat(7.0) / f64x8::splat(9.0) * t459 * t284 + f64x8::splat(2.0) / f64x8::splat(3.0) * t910 * t123 + f64x8::splat(7.0) / f64x8::splat(9.0) * t438 * t243 + f64x8::splat(0.5875309465443389) * t343 * t839 * t229 * t1715 - f64x8::splat(0.09792182442405649) * t350 * t839 * t20 * t251 - f64x8::splat(0.5875309465443389) * t626 * t37 * t839 * t628 * t1715 + f64x8::splat(0.023510491617264078) * t100 * t840 * t657 - f64x8::splat(8.0) / f64x8::splat(3.0) * t1730 * t20 * t123 - t588 * t425 * t460 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t926 * t375 - f64x8::splat(16.0) / f64x8::splat(9.0) * t920 * v_sigma * t284 - t588 * t432 * t180 / f64x8::splat(4.0) + f64x8::splat(32.0) / f64x8::splat(9.0) * t1745 * v_sigma * t272;
            let t1786 = t1380 * t425 * t451 / f64x8::splat(2.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t915 * t367 - f64x8::splat(8.0) / f64x8::splat(3.0) * t918 * t123 - f64x8::splat(2.4220232411039092e-05) * t247 / t18 / t517 * t1348 + f64x8::splat(14.0) / f64x8::splat(9.0) * t450 * t272 + t1552 * t168 * t180 + f64x8::splat(28.0) / f64x8::splat(9.0) * t454 * t280 - f64x8::splat(0.017017117742019715) * t257 * t359 * t261 - f64x8::splat(14.0) / f64x8::splat(9.0) * t427 * t231 + f64x8::splat(7.0) / f64x8::splat(9.0) * t434 * t237 - f64x8::splat(7.0) / f64x8::splat(9.0) * t456 * t280 - f64x8::splat(20.0) / f64x8::splat(3.0) * t378 * t284 + f64x8::splat(76.0) / f64x8::splat(3.0) * t891 * t765 * t49 - f64x8::splat(38.0) / f64x8::splat(9.0) * t169 * t248 * t27 * t49;
            let t1794 = t168 * t248;
            let t1824 = t224 * t261;
            let t1827 = -f64x8::splat(8.0) * t1170 * t876 + f64x8::splat(8.0) * t596 * t879 - f64x8::splat(4.0) / f64x8::splat(3.0) * t275 * t873 - f64x8::splat(76.0) / f64x8::splat(3.0) * t270 * t1794 * t229 + f64x8::splat(38.0) / f64x8::splat(9.0) * t120 * t1794 * t27 + f64x8::splat(8.0) / f64x8::splat(3.0) * t711 * t375 + f64x8::splat(40.0) / f64x8::splat(3.0) * t715 * t272 - f64x8::splat(8.0) / f64x8::splat(9.0) * t120 * t262 * t27 + f64x8::splat(64.0) / f64x8::splat(3.0) * t1359 * t261 * t1323 * t21 - f64x8::splat(32.0) * t627 * t261 * t628 * t21 + f64x8::splat(76.0) / f64x8::splat(3.0) * t627 * t1794 * t628 - f64x8::splat(76.0) / f64x8::splat(3.0) * t886 * t761 * t49 - f64x8::splat(16.0) / f64x8::splat(3.0) * t1567 * t367 - f64x8::splat(112.0) / f64x8::splat(9.0) * t1824 * t1334;
            let t1855 = t655 * t61 * v_sigma;
            let t1889 = f64x8::splat(0.0009382716049382716) * t62 * t262;
            let t1903 = t943 * t143;
            let t1907 = t474 * t312;
            let t1910 = t413 * t413;
            let t1916 = -f64x8::splat(24.0) * t552 * t402 * t413 - t144 * (-f64x8::splat(0.01273287037037037) * t481 * t317 + f64x8::splat(0.04963018212962963) * t484 * t320 - f64x8::splat(3.554682951960189) * t62 * t325 + f64x8::splat(1.5792627682922502) * t819 * v_sigma - f64x8::splat(1.7016662553370943) * t778 * t1273 * t63 * t1275 + t1889) - t542 * t490 - f64x8::splat(2.0) * t308 * t975 + f64x8::splat(2.0) * t1231 * t478 - f64x8::splat(2.0) * t1482 * t203 - f64x8::splat(4.0) * t790 * t413 - f64x8::splat(2.0) * t399 * t822 - f64x8::splat(2.0) * t1903 * t156 - t945 * t328 + f64x8::splat(2.0) * t1907 * t314 + f64x8::splat(4.0) * t313 * t1910 - f64x8::splat(6.0) * t552 * t490 * t314;
            let t1964 = f64x8::splat(24.0) * t1245 * t478 * t314 - f64x8::splat(12.0) * t1500 * t802 + f64x8::splat(8.0) * t1497 * t402 + f64x8::splat(8.0) * t793 * t805 + f64x8::splat(4.0) * t793 * t808 - f64x8::splat(12.0) * t1236 * t955 + f64x8::splat(8.0) * t545 * t958 - f64x8::splat(6.0) * t552 * t478 * t328 + f64x8::splat(4.0) * t313 * t203 * t822 + f64x8::splat(4.0) * t545 * t962 + f64x8::splat(4.0) * t313 * t975 * t156 + f64x8::splat(2.0) * t313 * t490 * t328 + (-f64x8::splat(388.6358024691358) * t773 * t465 + f64x8::splat(727.2376543209876) * t779 * t31 * t936 - f64x8::splat(646.4334705075446) * t291 / t18 / t294 / t259 * t59 * t61 * t292 * t1216 + f64x8::splat(3.537283950617284) * t62 * t287 - t1889) * t84;
            let t1967 = f64x8::splat(8.0) / f64x8::splat(9.0) * t99 * t261 * t1339 + f64x8::splat(32.0) / f64x8::splat(9.0) * t378 * t756 - f64x8::splat(64.0) / f64x8::splat(3.0) * t1320 * t261 * t1325 + f64x8::splat(32.0) * t626 * t261 * t1330 + f64x8::splat(64.0) / f64x8::splat(3.0) * t1638 * t762 - f64x8::splat(64.0) / f64x8::splat(3.0) * t715 * t766 + f64x8::splat(112.0) / f64x8::splat(9.0) * t270 * t827 * t21 + f64x8::splat(0.495729236146786) * t833 * t261 * t251 - f64x8::splat(0.4345280958817507) * t169 * t248 * t251 + f64x8::splat(0.021495306621498587) * t224 * t296 * t229 * t1855 - f64x8::splat(4.430586381716267e-07) * t1145 * t778 * t63 * t31 * t175 - f64x8::splat(0.991458472293572) * t1824 * t229 * t1290 - f64x8::splat(0.010747653310749294) * t99 * t296 * t27 * t1855 + f64x8::splat(6.509187460466756e-05) * t678 * t296 * v_sigma + t29 * (t1916 + t1964);
            let t1974 = ((t2).select(f64x8::splat(0.0), t6 * t216 * t494 / f64x8::splat(12.0) - t6 * t94 * t989 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1749 + t1786 + t1827 + t1967)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1974 + f64x8::splat(4.0) * t994;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t2007 = t655 * t61;
            let t2025 = f64x8::splat(12.0) * t627 * t425 * t129 * t628 * t21 + f64x8::splat(8.0) * t1320 * t425 * t129 * t1325 - f64x8::splat(12.0) * t626 * t425 * t129 * t1330 + f64x8::splat(14.0) / f64x8::splat(3.0) * t426 * t129 * t1334 - t437 * t129 * t1339 / f64x8::splat(3.0) + f64x8::splat(9.082587154139659e-06) * t101 / t18 / t295 * t27 * t677 - f64x8::splat(0.00806073998306197) * t224 * t387 * t229 * t2007 + f64x8::splat(0.004030369991530985) * t99 * t387 * t27 * t2007 - f64x8::splat(8.0) * t1359 * t425 * t129 * t1323 * t21 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1730 * t168 * t180 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1567 * t425 * t451;
            let t2051 = t169 * t981;
            let t2057 = f64x8::splat(3.0) / f64x8::splat(8.0) * t275 * t1004 * t460 - f64x8::splat(2.0) * t1018 * t367 + t1008 * t375 + f64x8::splat(3.0) / f64x8::splat(8.0) * t275 * t1000 * t180 + t1005 * t367 - t1011 * t375 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t596 * t1004 * t451 - f64x8::splat(3.0) / f64x8::splat(4.0) * t711 * t425 * t460 - f64x8::splat(3.0) / f64x8::splat(4.0) * t711 * t432 * t180 - f64x8::splat(0.2203241049541271) * t891 * t980 * t229 * t1284 + f64x8::splat(0.03672068415902118) * t2051 * t1284 - f64x8::splat(0.055081026238531776) * t437 * t130 * t251;
            let t2088 = f64x8::splat(0.11016205247706355) * t426 * t130 * t669 - f64x8::splat(0.004030369991530985) * t2051 * t657 + f64x8::splat(0.2203241049541271) * t886 * t980 * t628 * t1284 - f64x8::splat(4.0) * t1745 * t367 - f64x8::splat(12.0) * t1638 * t876 + f64x8::splat(12.0) * t715 * t879 - f64x8::splat(2.0) * t378 * t873 + t275 * t1063 / f64x8::splat(4.0) + f64x8::splat(2.0) * t920 * t375 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1170 * t1066 - f64x8::splat(3.0) / f64x8::splat(2.0) * t596 * t1070 + f64x8::splat(1.6614698931436002e-07) * t1145 * t932 * t36 * t38;
            let t2099 = t99 * t1060;
            let t2128 = t1033 * t143;
            let t2186 = -f64x8::splat(3.0) * t399 * t975 - f64x8::splat(6.0) * t1236 * t1041 + f64x8::splat(24.0) * t1245 * t1041 * t156 - f64x8::splat(18.0) * t552 * t478 * t413 + f64x8::splat(6.0) * t545 * t1044 - f64x8::splat(18.0) * t552 * t1044 * t156 + f64x8::splat(6.0) * t313 * t413 * t490 + f64x8::splat(6.0) * t313 * t203 * t975 - t308 * t1058 + f64x8::splat(2.0) * t313 * t1058 * t156 - t144 * (-f64x8::splat(0.008185416666666667) * t1047 * t145 + f64x8::splat(0.03190511708333333) * t1050 * t153 + f64x8::splat(0.05385883260545741) * t1053 * t969 - f64x8::splat(0.3528140227035878) * t972 + f64x8::splat(0.6381248457514104) * t932 * t1273 * t31 * t112 * t37);
            let t2189 = -f64x8::splat(0.055081026238531776) * t433 * t355 * t251 - f64x8::splat(1.3623880731209488e-05) * t678 * t387 + t1015 * t352 / f64x8::splat(2.0) + t997 * t123 - t1001 * t123 / f64x8::splat(2.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t2099 * t20 * t123 - f64x8::splat(14.0) / f64x8::splat(3.0) * t450 * t129 * t229 * t21 + t459 * t130 * t27 / f64x8::splat(3.0) - t1088 * t345 + t1092 * t105 / f64x8::splat(2.0) - f64x8::splat(2.0) * t1021 * t123 + t29 * ((-f64x8::splat(203.62654320987653) * t934 * t1028 + f64x8::splat(242.41255144032922) * t291 / t18 / t1147 * t59 * t359 * t389 + f64x8::splat(62.833333333333336) * t388 * t52) * t84 - t2128 * t156 - f64x8::splat(3.0) * t1903 * t203 + f64x8::splat(6.0) * t1907 * t402 - f64x8::splat(3.0) * t945 * t413 + f64x8::splat(6.0) * t1497 * t478 - f64x8::splat(18.0) * t1500 * t955 + f64x8::splat(12.0) * t793 * t958 - f64x8::splat(3.0) * t790 * t490 + f64x8::splat(6.0) * t793 * t962 + t2186);
            let t2196 = ((t2).select(f64x8::splat(0.0), -t6 * t94 * t1113 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t2025 + t2057 + t2088 + t2189)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t2196 + f64x8::splat(2.0) * t1117;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t2209 = f64x8::splat(1.0) / t292;
            let t2214 = f64x8::splat(1.0) / t1342;
            let t2268 = t433 * t1108;
            let t2275 = f64x8::splat(7.0) / f64x8::splat(4.0) * t1005 * t68 * t229 * t21 - t1011 * t1390 / f64x8::splat(8.0) + f64x8::splat(2.0) * t2099 * t168 * t180 + f64x8::splat(3.0) * t1745 * t425 * t451 + f64x8::splat(15.0) / f64x8::splat(8.0) * t270 * t2209 * t451 - f64x8::splat(15.0) / f64x8::splat(16.0) * t120 * t2209 * t460 - f64x8::splat(3.0) / f64x8::splat(2.0) * t920 * t425 * t460 - f64x8::splat(3.0) * t715 * t1004 * t451 + f64x8::splat(3.0) / f64x8::splat(2.0) * t378 * t1004 * t460 - f64x8::splat(0.0007556943734120596) * t2268 * t657 - f64x8::splat(0.08262153935779766) * t1073 * t1107 * t628 * t1284;
            let t2288 = t1000 * t443;
            let t2350 = t478 * t478;
            let t2356 = t490 * t490;
            let t2379 = (-f64x8::splat(90.90470679012346) * t291 * t1345 * t59 * t359 * t465 + f64x8::splat(43.63425925925926) * t1027 * t175 * t52) * t84 - f64x8::splat(4.0) * t2128 * t203 + f64x8::splat(12.0) * t1907 * t478 - f64x8::splat(6.0) * t945 * t490 - f64x8::splat(24.0) * t1500 * t1041 + f64x8::splat(24.0) * t793 * t1044 - f64x8::splat(4.0) * t399 * t1058 + f64x8::splat(24.0) * t1245 * t2350 - f64x8::splat(36.0) * t552 * t478 * t490 + f64x8::splat(6.0) * t313 * t2356 + f64x8::splat(8.0) * t313 * t203 * t1058 - t144 * (-f64x8::splat(0.01534765625) * t74 * t2214 * t80 + f64x8::splat(0.05982209453125) * t36 * t2209 * t200 + f64x8::splat(0.10098531113523264) * t62 * t1004 * t487 + f64x8::splat(0.018900751216263632) * t425 * t463 * t575 - f64x8::splat(0.2392968171567789) * t1025 * t1273 * t1275);
            let t2387 = -f64x8::splat(9.0) / f64x8::splat(2.0) * t224 * t1000 * t1080 + f64x8::splat(3.0) / f64x8::splat(4.0) * t1091 * t1084 - f64x8::splat(9.0) / f64x8::splat(2.0) * t627 * t2288 * t628 - f64x8::splat(0.04131076967889883) * t1091 * t443 * t251 + f64x8::splat(0.0030227774936482385) * t426 * t463 * t1158 - f64x8::splat(0.0015113887468241193) * t437 * t463 * t1164 - f64x8::splat(3.0) / f64x8::splat(2.0) * t920 * t432 * t180 + f64x8::splat(15.0) / f64x8::splat(16.0) * t99 * t2209 * t37 * t440 + t29 * t2379 - f64x8::splat(15.0) / f64x8::splat(16.0) * t120 * t2214 * t180 + f64x8::splat(3.0) / f64x8::splat(2.0) * t378 * t1000 * t180;
            let t2393 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1014 * t68 * t1339 / f64x8::splat(8.0) - f64x8::splat(3.405970182802372e-06) * t170 * t1134 * t27 * t677 - f64x8::splat(6.2305120992885e-08) * t1145 * t1025 * t1275 - f64x8::splat(15.0) / f64x8::splat(8.0) * t224 * t2209 * t37 * t429 + f64x8::splat(15.0) / f64x8::splat(16.0) * t99 * t2214 * t21 * t172 + f64x8::splat(3.0) * t1359 * t1004 * t68 * t1323 * t21 - f64x8::splat(9.0) / f64x8::splat(2.0) * t627 * t1004 * t68 * t628 * t21 - f64x8::splat(3.0) * t1320 * t1004 * t68 * t1325 + f64x8::splat(9.0) / f64x8::splat(2.0) * t626 * t1004 * t68 * t1330 - f64x8::splat(7.0) / f64x8::splat(4.0) * t1087 * t68 * t1334 + t2275 + f64x8::splat(0.08262153935779766) * t1078 * t1371 * t1284 - f64x8::splat(0.013770256559632944) * t2268 * t1284 + f64x8::splat(0.08262153935779766) * t1087 * t468 * t669 - f64x8::splat(0.04131076967889883) * t1014 * t468 * t251 + f64x8::splat(9.0) / f64x8::splat(2.0) * t270 * t2288 * t229 + f64x8::splat(9.0) / f64x8::splat(2.0) * t626 * t1000 * t1075 + f64x8::splat(6.0) * t1638 * t1066 - f64x8::splat(6.0) * t715 * t1070 + t378 * t1063 - f64x8::splat(3.0) / f64x8::splat(4.0) * t120 * t2288 * t27 + t2387)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t2393;
            acc_v4sigma4 = tv4sigma40;
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
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
