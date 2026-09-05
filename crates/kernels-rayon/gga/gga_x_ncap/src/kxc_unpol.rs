//! GGA_X_NCAP kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
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
pub fn gga_x_ncap_kxc_unpol(
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
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_zeta = f64x8::splat(param_zeta);
    let param_alpha = f64x8::splat(param_alpha);
    let param_beta = f64x8::splat(param_beta);
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
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t31 = t28 * t30;
            let t33 = t25 * t31 / f64x8::splat(12.0);
            let t34 = (simd::tanh(t33));
            let t35 = param_mu * t34;
            let t36 = (simd::ln(t33 + ((t33 * t33 + f64x8::splat(1.0)).sqrt())));
            let t37 = f64x8::splat(1.0) - param_zeta;
            let t39 = t37 * t21 * t24;
            let t40 = f64x8::splat(1.0) + t33;
            let t41 = (simd::ln(t40));
            let t42 = t30 * t41;
            let t46 = param_zeta * t21 * t24;
            let t51 = f64x8::splat(1.0) + param_alpha * (t39 * t28 * t42 / f64x8::splat(12.0) + t46 * t31 / f64x8::splat(12.0));
            let t52 = t36 * t51;
            let t53 = param_beta * t34;
            let t55 = t53 * t36 + f64x8::splat(1.0);
            let t56 = f64x8::splat(1.0) / t55;
            let t57 = t52 * t56;
            let t59 = t35 * t57 + f64x8::splat(1.0);
            let t63 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t59));
            let tzk0 = f64x8::splat(2.0) * t63;
            acc_zk = tzk0;
            let t64 = t18 * t18;
            let t66 = t17 / t64;
            let t70 = param_mu * t21;
            let t71 = t24 * t26;
            let t72 = t71 * t27;
            let t73 = t70 * t72;
            let t74 = v_rho * v_rho;
            let t76 = f64x8::splat(1.0) / t18 / t74;
            let t77 = t34 * t34;
            let t78 = f64x8::splat(1.0) - t77;
            let t79 = t76 * t78;
            let t80 = t79 * t57;
            let t84 = t35 * t25 * t26;
            let t85 = t27 * t76;
            let t86 = t23 * t23;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t20 * t87;
            let t89 = t27 * t27;
            let t90 = v_sigma * t89;
            let t92 = f64x8::splat(1.0) / t64 / t74;
            let t96 = f64x8::splat(6.0) * t88 * t90 * t92 + f64x8::splat(144.0);
            let t97 = ((t96).sqrt());
            let t98 = f64x8::splat(1.0) / t97;
            let t100 = t98 * t51 * t56;
            let t101 = t85 * t100;
            let t104 = t35 * t36;
            let t105 = t76 * t41;
            let t110 = t37 * t20 * t87;
            let t111 = t74 * v_rho;
            let t113 = f64x8::splat(1.0) / t64 / t111;
            let t114 = f64x8::splat(1.0) / t40;
            let t115 = t113 * t114;
            let t119 = t28 * t76;
            let t122 = -t39 * t28 * t105 / f64x8::splat(9.0) - t110 * t90 * t115 / f64x8::splat(18.0) - t46 * t119 / f64x8::splat(9.0);
            let t123 = param_alpha * t122;
            let t124 = t123 * t56;
            let t126 = t55 * t55;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t51 * t127;
            let t129 = param_beta * t21;
            let t130 = t129 * t71;
            let t131 = t78 * t36;
            let t132 = t85 * t131;
            let t135 = t53 * t25;
            let t136 = t76 * t98;
            let t140 = -t130 * t132 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t135 * t28 * t136;
            let t141 = t128 * t140;
            let t143 = -t73 * t80 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t101 + t104 * t124 - t104 * t141;
            let t148 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t59 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t143));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t148 + f64x8::splat(2.0) * t63;
            acc_vrho = tvrho0;
            let t151 = f64x8::splat(1.0) / t26;
            let t152 = t24 * t151;
            let t153 = t152 * t27;
            let t154 = t70 * t153;
            let t155 = t30 * t78;
            let t156 = t155 * t57;
            let t160 = t35 * t25 * t151;
            let t161 = t27 * t30;
            let t162 = t161 * t100;
            let t165 = t151 * t27;
            let t169 = t89 * t92;
            let t173 = t165 * t30;
            let t176 = t39 * t165 * t42 / f64x8::splat(24.0) + t110 * t169 * t114 / f64x8::splat(48.0) + t46 * t173 / f64x8::splat(24.0);
            let t177 = param_alpha * t176;
            let t178 = t177 * t56;
            let t180 = t129 * t152;
            let t181 = t161 * t131;
            let t184 = t30 * t98;
            let t188 = t180 * t181 / f64x8::splat(24.0) + t135 * t165 * t184 / f64x8::splat(2.0);
            let t189 = t128 * t188;
            let t191 = t154 * t156 / f64x8::splat(24.0) + t160 * t162 / f64x8::splat(2.0) + t104 * t178 - t104 * t189;
            let t195 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t191));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t195;
            acc_vsigma = tvsigma0;
            let t200 = t17 / t64 / v_rho;
            let t208 = f64x8::splat(1.0) / t18 / t111;
            let t209 = t208 * t78;
            let t210 = t209 * t57;
            let t213 = param_mu * t20;
            let t214 = t87 * v_sigma;
            let t216 = t213 * t214 * t89;
            let t217 = t74 * t74;
            let t219 = f64x8::splat(1.0) / t64 / t217;
            let t220 = t219 * t34;
            let t222 = t220 * t78 * t57;
            let t225 = t219 * t78;
            let t226 = t225 * t100;
            let t229 = t79 * t36;
            let t230 = t229 * t124;
            let t233 = t229 * t141;
            let t236 = t27 * t208;
            let t237 = t236 * t100;
            let t240 = f64x8::splat(1.0) / t22;
            let t241 = t26 * v_sigma;
            let t242 = t240 * t241;
            let t243 = t35 * t242;
            let t244 = t217 * t74;
            let t245 = f64x8::splat(1.0) / t244;
            let t247 = f64x8::splat(1.0) / t97 / t96;
            let t248 = t245 * t247;
            let t249 = t51 * t56;
            let t250 = t248 * t249;
            let t253 = t85 * t98;
            let t254 = t253 * t124;
            let t257 = t253 * t141;
            let t260 = t208 * t41;
            let t264 = t219 * t114;
            let t268 = t37 * t240;
            let t269 = t241 * t245;
            let t270 = t40 * t40;
            let t271 = f64x8::splat(1.0) / t270;
            let t275 = t28 * t208;
            let t278 = f64x8::splat(7.0) / f64x8::splat(27.0) * t39 * t28 * t260 + f64x8::splat(5.0) / f64x8::splat(18.0) * t110 * t90 * t264 - f64x8::splat(2.0) / f64x8::splat(27.0) * t268 * t269 * t271 + f64x8::splat(7.0) / f64x8::splat(27.0) * t46 * t275;
            let t279 = param_alpha * t278;
            let t280 = t279 * t56;
            let t282 = t127 * t140;
            let t283 = t123 * t282;
            let t287 = f64x8::splat(1.0) / t126 / t55;
            let t288 = t51 * t287;
            let t289 = t140 * t140;
            let t290 = t288 * t289;
            let t293 = t236 * t131;
            let t296 = param_beta * t20;
            let t297 = t296 * t214;
            let t298 = t89 * t219;
            let t299 = t34 * t78;
            let t300 = t299 * t36;
            let t301 = t298 * t300;
            let t304 = t78 * t98;
            let t305 = t298 * t304;
            let t308 = t208 * t98;
            let t312 = t53 * t240;
            let t316 = f64x8::splat(7.0) / f64x8::splat(27.0) * t130 * t293 - f64x8::splat(4.0) / f64x8::splat(27.0) * t297 * t301 + f64x8::splat(16.0) / f64x8::splat(9.0) * t297 * t305 + f64x8::splat(28.0) / f64x8::splat(9.0) * t135 * t28 * t308 - f64x8::splat(128.0) * t312 * t269 * t247;
            let t317 = t128 * t316;
            let t319 = f64x8::splat(7.0) / f64x8::splat(27.0) * t73 * t210 - f64x8::splat(4.0) / f64x8::splat(27.0) * t216 * t222 + f64x8::splat(16.0) / f64x8::splat(9.0) * t216 * t226 - f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t230 + f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t233 + f64x8::splat(28.0) / f64x8::splat(9.0) * t84 * t237 - f64x8::splat(128.0) * t243 * t250 - f64x8::splat(8.0) / f64x8::splat(3.0) * t84 * t254 + f64x8::splat(8.0) / f64x8::splat(3.0) * t84 * t257 + t104 * t280 - f64x8::splat(2.0) * t104 * t283 + f64x8::splat(2.0) * t104 * t290 - t104 * t317;
            let t324 = ((t2).select(f64x8::splat(0.0), t6 * t200 * t59 / f64x8::splat(12.0) - t6 * t66 * t143 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t319));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t324 + f64x8::splat(4.0) * t148;
            acc_v2rho2 = tv2rho20;
            let t332 = t87 * t89;
            let t334 = t213 * t332 * t113;
            let t335 = t299 * t57;
            let t338 = t213 * t332;
            let t339 = t113 * t78;
            let t340 = t339 * t100;
            let t343 = t155 * t36;
            let t344 = t343 * t124;
            let t347 = t343 * t141;
            let t352 = t240 * t26;
            let t353 = t35 * t352;
            let t354 = t217 * v_rho;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t355 * t247;
            let t357 = t356 * t249;
            let t360 = t161 * t98;
            let t361 = t360 * t124;
            let t364 = t360 * t141;
            let t367 = t229 * t178;
            let t370 = t253 * t178;
            let t376 = t89 * t113;
            let t384 = t165 * t76;
            let t387 = -t39 * t165 * t105 / f64x8::splat(18.0) - t110 * t376 * t114 / f64x8::splat(12.0) + t268 * t355 * t271 * t26 / f64x8::splat(36.0) - t46 * t384 / f64x8::splat(18.0);
            let t388 = param_alpha * t387;
            let t389 = t388 * t56;
            let t391 = t177 * t282;
            let t393 = t229 * t189;
            let t396 = t253 * t189;
            let t399 = t127 * t188;
            let t400 = t123 * t399;
            let t402 = t188 * t140;
            let t403 = t288 * t402;
            let t408 = t296 * t332;
            let t409 = t113 * t34;
            let t413 = t296 * t87;
            let t414 = t376 * t304;
            let t424 = -t180 * t132 / f64x8::splat(18.0) + t408 * t409 * t131 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t413 * t414 - f64x8::splat(2.0) / f64x8::splat(3.0) * t135 * t165 * t136 + f64x8::splat(48.0) * t312 * t26 * t355 * t247;
            let t425 = t128 * t424;
            let t427 = -t154 * t80 / f64x8::splat(18.0) + t334 * t335 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t338 * t340 + t154 * t344 / f64x8::splat(24.0) - t154 * t347 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t160 * t101 + f64x8::splat(48.0) * t353 * t357 + t160 * t361 / f64x8::splat(2.0) - t160 * t364 / f64x8::splat(2.0) - t73 * t367 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t370 + t104 * t389 - t104 * t391 + t73 * t393 / f64x8::splat(9.0) + f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t396 - t104 * t400 + f64x8::splat(2.0) * t104 * t403 - t104 * t425;
            let t432 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t191 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t427));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t432 + f64x8::splat(2.0) * t195;
            acc_v2rhosigma = tv2rhosigma0;
            let t435 = f64x8::splat(1.0) / t241;
            let t436 = t24 * t435;
            let t437 = t436 * t27;
            let t438 = t70 * t437;
            let t441 = f64x8::splat(1.0) / v_sigma;
            let t442 = t87 * t441;
            let t444 = t213 * t442 * t89;
            let t447 = t92 * t34 * t78 * t57;
            let t450 = t92 * t78;
            let t451 = t450 * t100;
            let t454 = t343 * t178;
            let t457 = t343 * t189;
            let t461 = t35 * t25 * t435;
            let t464 = t240 * t151;
            let t465 = t35 * t464;
            let t466 = f64x8::splat(1.0) / t217;
            let t467 = t466 * t247;
            let t468 = t467 * t249;
            let t471 = t360 * t178;
            let t473 = t360 * t189;
            let t475 = t435 * t27;
            let t479 = t441 * t89;
            let t480 = t92 * t114;
            let t488 = t475 * t30;
            let t491 = -t39 * t475 * t42 / f64x8::splat(48.0) + t110 * t479 * t480 / f64x8::splat(96.0) - t268 * t466 * t271 * t151 / f64x8::splat(96.0) - t46 * t488 / f64x8::splat(48.0);
            let t492 = param_alpha * t491;
            let t493 = t492 * t56;
            let t495 = t177 * t399;
            let t498 = t188 * t188;
            let t499 = t288 * t498;
            let t502 = t129 * t436;
            let t505 = t296 * t442;
            let t506 = t169 * t300;
            let t509 = t169 * t304;
            let t519 = -t502 * t181 / f64x8::splat(48.0) - t505 * t506 / f64x8::splat(48.0) + t505 * t509 / f64x8::splat(4.0) - t135 * t475 * t184 / f64x8::splat(4.0) - f64x8::splat(18.0) * t312 * t151 * t466 * t247;
            let t520 = t128 * t519;
            let t522 = -t438 * t156 / f64x8::splat(48.0) - t444 * t447 / f64x8::splat(48.0) + t444 * t451 / f64x8::splat(4.0) + t154 * t454 / f64x8::splat(12.0) - t154 * t457 / f64x8::splat(12.0) - t461 * t162 / f64x8::splat(4.0) - f64x8::splat(18.0) * t465 * t468 + t160 * t471 - t160 * t473 + t104 * t493 - f64x8::splat(2.0) * t104 * t495 + f64x8::splat(2.0) * t104 * t499 - t104 * t520;
            let t526 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t522));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t526;
            acc_v2sigma2 = tv2sigma20;
            let t529 = t17 * t92;
            let t540 = f64x8::splat(1.0) / t18 / t217;
            let t541 = t540 * t41;
            let t546 = f64x8::splat(1.0) / t64 / t354;
            let t551 = t217 * t111;
            let t552 = f64x8::splat(1.0) / t551;
            let t553 = t241 * t552;
            let t557 = v_sigma * v_sigma;
            let t558 = t217 * t217;
            let t560 = f64x8::splat(1.0) / t18 / t558;
            let t564 = f64x8::splat(1.0) / t270 / t40;
            let t566 = t24 * t27;
            let t567 = t564 * t21 * t566;
            let t574 = param_alpha * (-f64x8::splat(70.0) / f64x8::splat(81.0) * t39 * t28 * t541 - f64x8::splat(119.0) / f64x8::splat(81.0) * t110 * t90 * t546 * t114 + f64x8::splat(22.0) / f64x8::splat(27.0) * t268 * t553 * t271 - f64x8::splat(4.0) / f64x8::splat(243.0) * t268 * t557 * t560 * t567 - f64x8::splat(70.0) / f64x8::splat(81.0) * t46 * t28 * t540);
            let t575 = t574 * t56;
            let t577 = t27 * t540;
            let t578 = t577 * t131;
            let t581 = t89 * t546;
            let t585 = t581 * t304;
            let t588 = param_beta * t240;
            let t589 = t588 * t241;
            let t590 = t78 * t78;
            let t595 = t552 * t77;
            let t599 = t552 * t34;
            let t604 = f64x8::splat(1.0) / t23 / t22;
            let t605 = t604 * t557;
            let t606 = t129 * t605;
            let t607 = t27 * t560;
            let t608 = t78 * t247;
            let t612 = t540 * t98;
            let t619 = t24 * t557;
            let t620 = t129 * t619;
            let t621 = t78 * t240;
            let t622 = t621 * t247;
            let t623 = t607 * t622;
            let t626 = t26 * t557;
            let t627 = t240 * t626;
            let t628 = t53 * t627;
            let t629 = t558 * v_rho;
            let t631 = f64x8::splat(1.0) / t64 / t629;
            let t632 = t96 * t96;
            let t634 = f64x8::splat(1.0) / t97 / t632;
            let t636 = t88 * t89;
            let t637 = t631 * t634 * t636;
            let t640 = -f64x8::splat(70.0) / f64x8::splat(81.0) * t130 * t578 + f64x8::splat(28.0) / f64x8::splat(27.0) * t297 * t581 * t300 - f64x8::splat(112.0) / f64x8::splat(9.0) * t297 * t585 + f64x8::splat(16.0) / f64x8::splat(81.0) * t589 * t552 * t590 * t36 - f64x8::splat(32.0) / f64x8::splat(81.0) * t589 * t595 * t131 + f64x8::splat(64.0) / f64x8::splat(9.0) * t589 * t599 * t304 + f64x8::splat(256.0) / f64x8::splat(9.0) * t606 * t607 * t608 - f64x8::splat(280.0) / f64x8::splat(27.0) * t135 * t28 * t612 + f64x8::splat(3200.0) / f64x8::splat(3.0) * t312 * t553 * t247 + f64x8::splat(128.0) / f64x8::splat(9.0) * t620 * t623 - f64x8::splat(3072.0) * t628 * t637;
            let t641 = t128 * t640;
            let t643 = t126 * t126;
            let t644 = f64x8::splat(1.0) / t643;
            let t645 = t51 * t644;
            let t646 = t289 * t140;
            let t647 = t645 * t646;
            let t650 = param_mu * t240;
            let t651 = t650 * t553;
            let t652 = t590 * t36;
            let t653 = t652 * t249;
            let t656 = t552 * t247;
            let t657 = t656 * t249;
            let t660 = t546 * t78;
            let t661 = t660 * t100;
            let t665 = t70 * t605 * t27;
            let t666 = t560 * t78;
            let t667 = t247 * t51;
            let t668 = t667 * t56;
            let t672 = t577 * t100;
            let t675 = t540 * t78;
            let t676 = t675 * t57;
            let t679 = t225 * t98;
            let t686 = t229 * t280;
            let t689 = t209 * t36;
            let t690 = t689 * t141;
            let t693 = t546 * t34;
            let t698 = t689 * t124;
            let t701 = t253 * t290;
            let t704 = t229 * t290;
            let t707 = t253 * t317;
            let t710 = t104 * t575 - t104 * t641 - f64x8::splat(6.0) * t104 * t647 + f64x8::splat(16.0) / f64x8::splat(81.0) * t651 * t653 + f64x8::splat(3200.0) / f64x8::splat(3.0) * t243 * t657 - f64x8::splat(112.0) / f64x8::splat(9.0) * t216 * t661 + f64x8::splat(256.0) / f64x8::splat(9.0) * t665 * t666 * t668 - f64x8::splat(280.0) / f64x8::splat(27.0) * t84 * t672 - f64x8::splat(70.0) / f64x8::splat(81.0) * t73 * t676 - f64x8::splat(16.0) / f64x8::splat(3.0) * t216 * t679 * t141 + f64x8::splat(16.0) / f64x8::splat(3.0) * t216 * t679 * t124 - t73 * t686 / f64x8::splat(3.0) - f64x8::splat(7.0) / f64x8::splat(9.0) * t73 * t690 + f64x8::splat(28.0) / f64x8::splat(27.0) * t216 * t693 * t78 * t57 + f64x8::splat(7.0) / f64x8::splat(9.0) * t73 * t698 - f64x8::splat(8.0) * t84 * t701 - f64x8::splat(2.0) / f64x8::splat(3.0) * t73 * t704 + f64x8::splat(4.0) * t84 * t707;
            let t711 = t229 * t317;
            let t714 = t236 * t98;
            let t715 = t714 * t141;
            let t718 = t714 * t124;
            let t722 = t70 * t619 * t27;
            let t724 = t666 * t240 * t668;
            let t729 = t634 * t51;
            let t731 = t729 * t56 * t636;
            let t734 = t253 * t280;
            let t737 = t35 * t21;
            let t738 = t737 * t72;
            let t739 = t136 * param_alpha;
            let t740 = t122 * t127;
            let t741 = t740 * t140;
            let t742 = t739 * t741;
            let t745 = t213 * t87;
            let t747 = t745 * t90 * t219;
            let t748 = t300 * t141;
            let t751 = t300 * t124;
            let t754 = t70 * t24;
            let t755 = t754 * t119;
            let t756 = t131 * param_alpha;
            let t757 = t756 * t741;
            let t763 = t127 * t316;
            let t767 = t140 * t316;
            let t771 = t287 * t289;
            let t776 = t77 * t78 * t57;
            let t779 = t299 * t100;
            let t782 = t248 * t141;
            let t785 = t248 * t124;
            let t788 = t73 * t711 / f64x8::splat(3.0) - f64x8::splat(28.0) / f64x8::splat(3.0) * t84 * t715 + f64x8::splat(28.0) / f64x8::splat(3.0) * t84 * t718 + f64x8::splat(128.0) / f64x8::splat(9.0) * t722 * t724 - f64x8::splat(3072.0) * t35 * t627 * t631 * t731 - f64x8::splat(4.0) * t84 * t734 + f64x8::splat(8.0) * t738 * t742 + f64x8::splat(4.0) / f64x8::splat(9.0) * t747 * t748 - f64x8::splat(4.0) / f64x8::splat(9.0) * t747 * t751 + f64x8::splat(2.0) / f64x8::splat(3.0) * t755 * t757 - f64x8::splat(3.0) * t104 * t279 * t282 - f64x8::splat(3.0) * t104 * t123 * t763 + f64x8::splat(6.0) * t104 * t288 * t767 + f64x8::splat(6.0) * t104 * t123 * t771 - f64x8::splat(32.0) / f64x8::splat(81.0) * t651 * t776 + f64x8::splat(64.0) / f64x8::splat(9.0) * t651 * t779 + f64x8::splat(384.0) * t243 * t782 - f64x8::splat(384.0) * t243 * t785;
            let t789 = t710 + t788;
            let t794 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t529 * t59 + t6 * t200 * t143 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t66 * t319 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t789));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t794 + f64x8::splat(6.0) * t324;
            acc_v3rho3 = tv3rho30;
            let t808 = t127 * t424;
            let t812 = t424 * t140;
            let t816 = t188 * t316;
            let t825 = t245 * t77;
            let t827 = t26 * t78;
            let t828 = t827 * t57;
            let t831 = t245 * t34;
            let t833 = t827 * t100;
            let t836 = t356 * t141;
            let t839 = t356 * t124;
            let t842 = t248 * t178;
            let t845 = t248 * t189;
            let t859 = f64x8::splat(1.0) / t18 / t551;
            let t863 = v_sigma * t21 * t566;
            let t870 = param_alpha * (f64x8::splat(7.0) / f64x8::splat(54.0) * t39 * t165 * t260 + f64x8::splat(37.0) / f64x8::splat(108.0) * t110 * t298 * t114 - t268 * t245 * t271 * t26 / f64x8::splat(4.0) + t268 * t859 * t564 * t863 / f64x8::splat(162.0) + f64x8::splat(7.0) / f64x8::splat(54.0) * t46 * t165 * t208);
            let t871 = t870 * t56;
            let t880 = t588 * t245;
            let t882 = t26 * t590 * t36;
            let t886 = t77 * t26 * t131;
            let t890 = t299 * t26 * t98;
            let t893 = t604 * t27;
            let t894 = t129 * t893;
            let t895 = t859 * t78;
            let t896 = t247 * v_sigma;
            let t903 = t26 * t245;
            let t907 = t24 * v_sigma;
            let t908 = t129 * t907;
            let t909 = t27 * t859;
            let t913 = t53 * t242;
            let t915 = f64x8::splat(1.0) / t64 / t558;
            let t917 = t915 * t634 * t636;
            let t920 = f64x8::splat(7.0) / f64x8::splat(54.0) * t180 * t293 - f64x8::splat(5.0) / f64x8::splat(18.0) * t408 * t220 * t131 + f64x8::splat(10.0) / f64x8::splat(3.0) * t413 * t305 - f64x8::splat(2.0) / f64x8::splat(27.0) * t880 * t882 + f64x8::splat(4.0) / f64x8::splat(27.0) * t880 * t886 - f64x8::splat(8.0) / f64x8::splat(3.0) * t880 * t890 - f64x8::splat(32.0) / f64x8::splat(3.0) * t894 * t895 * t896 + f64x8::splat(14.0) / f64x8::splat(9.0) * t135 * t165 * t308 - f64x8::splat(304.0) * t312 * t903 * t247 - f64x8::splat(16.0) / f64x8::splat(3.0) * t908 * t909 * t622 + f64x8::splat(1152.0) * t913 * t917;
            let t921 = t128 * t920;
            let t923 = -t104 * t177 * t763 - t104 * t279 * t399 - f64x8::splat(2.0) * t104 * t123 * t808 + f64x8::splat(4.0) * t104 * t288 * t812 + f64x8::splat(2.0) * t104 * t288 * t816 - f64x8::splat(2.0) * t104 * t388 * t282 + f64x8::splat(10.0) / f64x8::splat(3.0) * t338 * t226 + f64x8::splat(4.0) / f64x8::splat(27.0) * t650 * t825 * t828 - f64x8::splat(8.0) / f64x8::splat(3.0) * t650 * t831 * t833 - f64x8::splat(96.0) * t353 * t836 + f64x8::splat(96.0) * t353 * t839 - f64x8::splat(128.0) * t243 * t842 + f64x8::splat(128.0) * t243 * t845 + t104 * t871 - t104 * t921;
            let t925 = t35 * t36 * param_alpha;
            let t926 = t122 * t287;
            let t927 = t926 * t402;
            let t930 = t650 * t903;
            let t935 = t304 * t141;
            let t939 = t70 * t893 * t859;
            let t941 = t608 * t249 * v_sigma;
            let t944 = t304 * t124;
            let t948 = t213 * t332 * t219;
            let t967 = f64x8::splat(4.0) * t925 * t927 - f64x8::splat(2.0) / f64x8::splat(27.0) * t930 * t653 - f64x8::splat(304.0) * t353 * t250 + f64x8::splat(4.0) / f64x8::splat(3.0) * t334 * t935 - f64x8::splat(32.0) / f64x8::splat(3.0) * t939 * t941 - f64x8::splat(4.0) / f64x8::splat(3.0) * t334 * t944 - f64x8::splat(5.0) / f64x8::splat(18.0) * t948 * t335 + f64x8::splat(14.0) / f64x8::splat(9.0) * t160 * t237 + f64x8::splat(7.0) / f64x8::splat(54.0) * t154 * t210 + f64x8::splat(4.0) / f64x8::splat(3.0) * t160 * t257 - f64x8::splat(4.0) / f64x8::splat(3.0) * t160 * t254 + t154 * t233 / f64x8::splat(9.0) - t154 * t230 / f64x8::splat(9.0) - t334 * t748 / f64x8::splat(9.0) + t334 * t751 / f64x8::splat(9.0);
            let t969 = t714 * t189;
            let t972 = t360 * t290;
            let t974 = t689 * t178;
            let t977 = t714 * t178;
            let t980 = t343 * t290;
            let t983 = t253 * t389;
            let t986 = t229 * t425;
            let t992 = t253 * t425;
            let t996 = t70 * t907 * t27;
            let t1005 = t360 * t280;
            let t1008 = t360 * t317;
            let t1011 = t229 * t389;
            let t1017 = -f64x8::splat(28.0) / f64x8::splat(9.0) * t84 * t969 + t160 * t972 + f64x8::splat(7.0) / f64x8::splat(27.0) * t73 * t974 + f64x8::splat(28.0) / f64x8::splat(9.0) * t84 * t977 + t154 * t980 / f64x8::splat(12.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t84 * t983 + f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t986 - f64x8::splat(16.0) / f64x8::splat(9.0) * t216 * t679 * t189 + f64x8::splat(8.0) / f64x8::splat(3.0) * t84 * t992 - f64x8::splat(16.0) / f64x8::splat(3.0) * t996 * t895 * t240 * t668 + f64x8::splat(1152.0) * t35 * t242 * t915 * t731 + t160 * t1005 / f64x8::splat(2.0) - t160 * t1008 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t1011 + f64x8::splat(16.0) / f64x8::splat(9.0) * t216 * t679 * t178;
            let t1018 = t343 * t280;
            let t1021 = t343 * t317;
            let t1024 = t689 * t189;
            let t1027 = t754 * t173;
            let t1030 = t136 * t51;
            let t1031 = t287 * t188;
            let t1032 = t1031 * t140;
            let t1033 = t1030 * t1032;
            let t1036 = t740 * t188;
            let t1037 = t739 * t1036;
            let t1040 = t131 * t51;
            let t1041 = t1040 * t1032;
            let t1044 = t300 * t189;
            let t1047 = t756 * t1036;
            let t1050 = t176 * t127;
            let t1051 = t1050 * t140;
            let t1052 = t756 * t1051;
            let t1055 = t300 * t178;
            let t1058 = t739 * t1051;
            let t1061 = t737 * t153;
            let t1062 = t184 * param_alpha;
            let t1063 = t1062 * t741;
            let t1068 = t188 * t289;
            let t1072 = t154 * t1018 / f64x8::splat(24.0) - t154 * t1021 / f64x8::splat(24.0) - f64x8::splat(7.0) / f64x8::splat(27.0) * t73 * t1024 - t1027 * t757 / f64x8::splat(12.0) - f64x8::splat(16.0) / f64x8::splat(3.0) * t738 * t1033 + f64x8::splat(8.0) / f64x8::splat(3.0) * t738 * t1037 - f64x8::splat(4.0) / f64x8::splat(9.0) * t755 * t1041 + f64x8::splat(4.0) / f64x8::splat(27.0) * t747 * t1044 + f64x8::splat(2.0) / f64x8::splat(9.0) * t755 * t1047 + f64x8::splat(2.0) / f64x8::splat(9.0) * t755 * t1052 - f64x8::splat(4.0) / f64x8::splat(27.0) * t747 * t1055 + f64x8::splat(8.0) / f64x8::splat(3.0) * t738 * t1058 - t1061 * t1063 + f64x8::splat(2.0) * t104 * t177 * t771 - f64x8::splat(6.0) * t104 * t645 * t1068;
            let t1074 = t923 + t967 + t1017 + t1072;
            let t1079 = ((t2).select(f64x8::splat(0.0), t6 * t200 * t191 / f64x8::splat(12.0) - t6 * t66 * t427 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1074));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t1079 + f64x8::splat(4.0) * t432;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t1091 = t151 * t355;
            let t1096 = f64x8::splat(1.0) / t18 / t244;
            let t1104 = param_alpha * (t39 * t475 * t105 / f64x8::splat(36.0) - t110 * t479 * t115 / f64x8::splat(72.0) + t268 * t1091 * t271 / f64x8::splat(18.0) - t268 * t1096 * t567 / f64x8::splat(432.0) + t46 * t475 * t76 / f64x8::splat(36.0));
            let t1105 = t1104 * t56;
            let t1109 = t376 * t300;
            let t1114 = t588 * t151;
            let t1126 = t129 * t604;
            let t1127 = t27 * t1096;
            let t1128 = t1127 * t608;
            let t1137 = t129 * t566;
            let t1138 = t1096 * t78;
            let t1139 = t240 * t247;
            let t1143 = t53 * t352;
            let t1145 = f64x8::splat(1.0) / t64 / t551;
            let t1147 = t1145 * t634 * t636;
            let t1150 = t502 * t132 / f64x8::splat(36.0) + t505 * t1109 / f64x8::splat(36.0) - t505 * t414 / f64x8::splat(3.0) + t1114 * t355 * t590 * t36 / f64x8::splat(36.0) - t1114 * t355 * t77 * t131 / f64x8::splat(18.0) + t1114 * t355 * t34 * t304 + f64x8::splat(4.0) * t1126 * t1128 + t135 * t475 * t136 / f64x8::splat(3.0) + f64x8::splat(48.0) * t312 * t1091 * t247 + f64x8::splat(2.0) * t1137 * t1138 * t1139 - f64x8::splat(432.0) * t1143 * t1147;
            let t1151 = t128 * t1150;
            let t1153 = t650 * t1091;
            let t1158 = t176 * t287;
            let t1159 = t1158 * t402;
            let t1162 = t287 * t498;
            let t1166 = t519 * t140;
            let t1170 = t127 * t519;
            let t1181 = t498 * t140;
            let t1185 = t188 * t424;
            let t1192 = t104 * t1105 - t104 * t1151 + t1153 * t653 / f64x8::splat(36.0) + f64x8::splat(48.0) * t465 * t357 + f64x8::splat(4.0) * t925 * t1159 + f64x8::splat(2.0) * t104 * t123 * t1162 + f64x8::splat(2.0) * t104 * t288 * t1166 - t104 * t123 * t1170 - t104 * t492 * t282 - f64x8::splat(2.0) * t104 * t388 * t399 - f64x8::splat(2.0) * t104 * t177 * t808 - f64x8::splat(6.0) * t104 * t645 * t1181 + f64x8::splat(4.0) * t104 * t288 * t1185 - t1153 * t776 / f64x8::splat(18.0) + t1153 * t779;
            let t1193 = t70 * t893;
            let t1194 = t1138 * t668;
            let t1197 = t467 * t141;
            let t1200 = t467 * t124;
            let t1203 = t356 * t178;
            let t1206 = t356 * t189;
            let t1211 = t304 * t178;
            let t1214 = t304 * t189;
            let t1218 = t70 * t566 * t1096;
            let t1219 = t621 * t668;
            let t1228 = t450 * t98;
            let t1229 = t1228 * t141;
            let t1232 = t1228 * t124;
            let t1239 = f64x8::splat(4.0) * t1193 * t1194 + f64x8::splat(18.0) * t465 * t1197 - f64x8::splat(18.0) * t465 * t1200 + f64x8::splat(96.0) * t353 * t1203 - f64x8::splat(96.0) * t353 * t1206 - t444 * t340 / f64x8::splat(3.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t334 * t1211 + f64x8::splat(4.0) / f64x8::splat(3.0) * t334 * t1214 + f64x8::splat(2.0) * t1218 * t1219 + t461 * t101 / f64x8::splat(3.0) + t438 * t80 / f64x8::splat(36.0) - t438 * t344 / f64x8::splat(48.0) - t444 * t1229 / f64x8::splat(4.0) + t444 * t1232 / f64x8::splat(4.0) - f64x8::splat(432.0) * t35 * t352 * t1145 * t731;
            let t1241 = t360 * t389;
            let t1243 = t360 * t425;
            let t1245 = t229 * t493;
            let t1248 = t253 * t493;
            let t1251 = t229 * t499;
            let t1254 = t253 * t499;
            let t1257 = t229 * t520;
            let t1260 = t253 * t520;
            let t1263 = t343 * t389;
            let t1268 = t343 * t425;
            let t1279 = t160 * t1241 - t160 * t1243 - t73 * t1245 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t1248 - f64x8::splat(2.0) / f64x8::splat(9.0) * t73 * t1251 - f64x8::splat(8.0) / f64x8::splat(3.0) * t84 * t1254 + t73 * t1257 / f64x8::splat(9.0) + f64x8::splat(4.0) / f64x8::splat(3.0) * t84 * t1260 + t154 * t1263 / f64x8::splat(12.0) + t334 * t1055 / f64x8::splat(9.0) - t154 * t1268 / f64x8::splat(12.0) - t334 * t1044 / f64x8::splat(9.0) + t461 * t364 / f64x8::splat(4.0) - t461 * t361 / f64x8::splat(4.0) + t438 * t347 / f64x8::splat(48.0);
            let t1281 = t409 * t78 * t57;
            let t1292 = t184 * t51;
            let t1293 = t1292 * t1032;
            let t1296 = t1062 * t1036;
            let t1298 = t1050 * t188;
            let t1299 = t756 * t1298;
            let t1302 = t739 * t1298;
            let t1309 = t1062 * t1051;
            let t1312 = t745 * t479 * t92;
            let t1319 = t444 * t1281 / f64x8::splat(36.0) + f64x8::splat(4.0) / f64x8::splat(3.0) * t160 * t396 + t154 * t393 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t160 * t370 - t154 * t367 / f64x8::splat(9.0) + f64x8::splat(2.0) * t1061 * t1293 - t1061 * t1296 + f64x8::splat(2.0) / f64x8::splat(9.0) * t755 * t1299 + f64x8::splat(8.0) / f64x8::splat(3.0) * t738 * t1302 + t1027 * t1041 / f64x8::splat(6.0) - t1027 * t1047 / f64x8::splat(12.0) - t1061 * t1309 + t1312 * t748 / f64x8::splat(48.0) - t1312 * t751 / f64x8::splat(48.0) - t1027 * t1052 / f64x8::splat(12.0);
            let t1321 = t1192 + t1239 + t1279 + t1319;
            let t1326 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t522 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1321));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t1326 + f64x8::splat(2.0) * t526;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t1329 = t498 * t188;
            let t1330 = t645 * t1329;
            let t1333 = f64x8::splat(1.0) / t626;
            let t1334 = t24 * t1333;
            let t1335 = t129 * t1334;
            let t1338 = f64x8::splat(1.0) / t557;
            let t1339 = t87 * t1338;
            let t1340 = t296 * t1339;
            let t1345 = t588 * t435;
            let t1347 = t466 * t590 * t36;
            let t1351 = t466 * t77 * t131;
            let t1355 = t466 * t34 * t304;
            let t1358 = t604 * t441;
            let t1359 = t129 * t1358;
            let t1361 = f64x8::splat(1.0) / t18 / t354;
            let t1362 = t27 * t1361;
            let t1363 = t1362 * t608;
            let t1366 = t1333 * t27;
            let t1370 = t435 * t466;
            let t1374 = t24 * t441;
            let t1375 = t129 * t1374;
            let t1376 = t1362 * t622;
            let t1379 = t53 * t464;
            let t1381 = f64x8::splat(1.0) / t64 / t244;
            let t1383 = t1381 * t634 * t636;
            let t1386 = t1335 * t181 / f64x8::splat(32.0) + t1340 * t506 / f64x8::splat(32.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1340 * t509 - t1345 * t1347 / f64x8::splat(96.0) + t1345 * t1351 / f64x8::splat(48.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1345 * t1355 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1359 * t1363 + f64x8::splat(3.0) / f64x8::splat(8.0) * t135 * t1366 * t184 + f64x8::splat(18.0) * t312 * t1370 * t247 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1375 * t1376 + f64x8::splat(162.0) * t1379 * t1383;
            let t1387 = t128 * t1386;
            let t1392 = t1338 * t89;
            let t1397 = t268 * t1361 * t564;
            let t1399 = t441 * t21 * t566;
            let t1406 = param_alpha * (t39 * t1366 * t42 / f64x8::splat(32.0) - t110 * t1392 * t480 / f64x8::splat(64.0) + t1397 * t1399 / f64x8::splat(1152.0) + t46 * t1366 * t30 / f64x8::splat(32.0));
            let t1407 = t1406 * t56;
            let t1409 = t650 * t1370;
            let t1412 = t240 * t435;
            let t1413 = t35 * t1412;
            let t1419 = t188 * t519;
            let t1433 = t467 * t189;
            let t1436 = t467 * t178;
            let t1440 = t213 * t1339 * t89;
            let t1444 = t70 * t1358 * t27;
            let t1445 = t1361 * t78;
            let t1446 = t1445 * t668;
            let t1450 = t35 * t25 * t1333;
            let t1454 = t70 * t1334 * t27;
            let t1459 = -f64x8::splat(6.0) * t104 * t1330 - t104 * t1387 + t104 * t1407 - t1409 * t653 / f64x8::splat(96.0) + f64x8::splat(18.0) * t1413 * t468 - f64x8::splat(3.0) * t104 * t177 * t1170 + f64x8::splat(6.0) * t104 * t288 * t1419 - f64x8::splat(3.0) * t104 * t492 * t399 + f64x8::splat(6.0) * t104 * t177 * t1162 + t1409 * t776 / f64x8::splat(48.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1409 * t779 + f64x8::splat(54.0) * t465 * t1433 - f64x8::splat(54.0) * t465 * t1436 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1440 * t451 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1444 * t1446 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1450 * t162 + t1454 * t156 / f64x8::splat(32.0) - t438 * t454 / f64x8::splat(16.0);
            let t1460 = t360 * t499;
            let t1463 = t343 * t499;
            let t1471 = t70 * t1374 * t27;
            let t1473 = t1445 * t240 * t668;
            let t1480 = t360 * t493;
            let t1483 = t360 * t520;
            let t1486 = t1228 * t189;
            let t1489 = t1228 * t178;
            let t1492 = t343 * t493;
            let t1495 = t343 * t520;
            let t1508 = t1062 * t1298;
            let t1511 = f64x8::splat(3.0) * t160 * t1460 + t154 * t1463 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t461 * t473 - f64x8::splat(3.0) / f64x8::splat(4.0) * t461 * t471 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1471 * t1473 + f64x8::splat(162.0) * t35 * t464 * t1381 * t731 + f64x8::splat(3.0) / f64x8::splat(2.0) * t160 * t1480 - f64x8::splat(3.0) / f64x8::splat(2.0) * t160 * t1483 - f64x8::splat(3.0) / f64x8::splat(4.0) * t444 * t1486 + f64x8::splat(3.0) / f64x8::splat(4.0) * t444 * t1489 + t154 * t1492 / f64x8::splat(8.0) - t154 * t1495 / f64x8::splat(8.0) + t438 * t457 / f64x8::splat(16.0) + t1440 * t447 / f64x8::splat(32.0) + t1312 * t1044 / f64x8::splat(16.0) - t1312 * t1055 / f64x8::splat(16.0) - t1027 * t1299 / f64x8::splat(4.0) - f64x8::splat(3.0) * t1061 * t1508;
            let t1512 = t1459 + t1511;
            let t1516 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t1512));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t1516;
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
