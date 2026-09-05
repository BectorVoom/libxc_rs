//! MGGA_X_RPPSCAN fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`
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
pub fn mgga_x_rppscan_fxc_unpol(
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
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t20 * t20;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t39 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t40 = t21 * t21;
            let t42 = t23 * t22;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t40 * t43;
            let t45 = v_sigma * v_sigma;
            let t46 = t45 * t27;
            let t47 = t30 * t30;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t20 / t48;
            let t55 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t39 * t21 * t25 * t34));
            let t56 = t50 * t55;
            let t60 = ((f64x8::splat(146.0)).sqrt());
            let t61 = t60 * t21;
            let t62 = t61 * t25;
            let t65 = v_tau * t28;
            let t66 = t31 * v_rho;
            let t67 = f64x8::splat(1.0) / t66;
            let t70 = t65 * t67 - t34 / f64x8::splat(8.0);
            let t73 = param_eta * v_sigma;
            let t74 = t28 * t33;
            let t77 = f64x8::splat(3.0) / f64x8::splat(10.0) * t40 * t24 + t73 * t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t70 * t78;
            let t80 = f64x8::splat(1.0) - t79;
            let t82 = t80 * t80;
            let t84 = (simd::exp(-t82 / f64x8::splat(2.0)));
            let t87 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t80 * t84 / f64x8::splat(100.0);
            let t88 = t87 * t87;
            let t89 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t88;
            let t94 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t89);
            let t95 = (t79).simd_le(f64x8::splat(2.5));
            let t96 = (f64x8::splat(2.5)).simd_lt(t79);
            let t97 = ((t96).select(f64x8::splat(2.5), t79));
            let t99 = t97 * t97;
            let t101 = t99 * t97;
            let t103 = t99 * t99;
            let t105 = t103 * t97;
            let t107 = t103 * t99;
            let t112 = ((t96).select(t79, f64x8::splat(2.5)));
            let t113 = f64x8::splat(1.0) - t112;
            let t116 = (simd::exp(param_c2 / t113));
            let t118 = ((t95).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t97 - f64x8::splat(0.4445555) * t99 - f64x8::splat(0.663086601049) * t101 + f64x8::splat(1.45129704449) * t103 - f64x8::splat(0.887998041597) * t105 + f64x8::splat(0.234528941479) * t107 - f64x8::splat(0.023185843322) * t103 * t101, -param_d * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t122 = t94 * t119 + f64x8::splat(1.174) * t118;
            let t124 = ((f64x8::splat(3.0)).sqrt());
            let t125 = f64x8::splat(1.0) / t23;
            let t126 = t40 * t125;
            let t127 = ((v_sigma).sqrt());
            let t128 = t127 * t27;
            let t130 = f64x8::splat(1.0) / t20 / v_rho;
            let t132 = t126 * t128 * t130;
            let t133 = ((t132).sqrt());
            let t137 = (simd::exp(-f64x8::splat(9.8958) * t124 / t133));
            let t138 = f64x8::splat(1.0) - t137;
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t122 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t31;
            let t148 = param_k1 * param_k1;
            let t149 = t89 * t89;
            let t151 = t148 / t149;
            let t152 = t30 * v_rho;
            let t154 = f64x8::splat(1.0) / t31 / t152;
            let t155 = t29 * t154;
            let t158 = t47 * t30;
            let t160 = f64x8::splat(1.0) / t20 / t158;
            let t161 = t160 * t55;
            let t165 = t39 * t39;
            let t166 = t22 * t22;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t169 = t45 * v_sigma;
            let t170 = t47 * t47;
            let t171 = t170 * v_rho;
            let t172 = f64x8::splat(1.0) / t171;
            let t182 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t65 * t33 + t155 / f64x8::splat(3.0);
            let t184 = t77 * t77;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t70 * t185;
            let t187 = t186 * param_eta;
            let t190 = -t182 * t78 - t187 * t155 / f64x8::splat(3.0);
            let t194 = t60 * t82;
            let t195 = t190 * t84;
            let t198 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t62 * t155 + t60 * t190 * t84 / f64x8::splat(100.0) - t194 * t195 / f64x8::splat(100.0);
            let t201 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t155 - t44 * t46 * t161 / f64x8::splat(54.0) + f64x8::splat(3.0) / f64x8::splat(80.0) * t168 * t169 * t172 * t55 + f64x8::splat(2.0) * t87 * t198;
            let t202 = t201 * t119;
            let t204 = -t190;
            let t205 = ((t96).select(f64x8::splat(0.0), t204));
            let t207 = t97 * t205;
            let t209 = t99 * t205;
            let t211 = t101 * t205;
            let t213 = t103 * t205;
            let t215 = t105 * t205;
            let t220 = param_d * param_c2;
            let t221 = t113 * t113;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = ((t96).select(t204, f64x8::splat(0.0)));
            let t227 = ((t95).select(-f64x8::splat(0.667) * t205 - f64x8::splat(0.889111) * t207 - f64x8::splat(1.989259803147) * t209 + f64x8::splat(5.80518817796) * t211 - f64x8::splat(4.439990207985) * t213 + f64x8::splat(1.407173648874) * t215 - f64x8::splat(0.162300903254) * t107 * t205, -t220 * t222 * t223 * t116));
            let t230 = t151 * t202 - t94 * t227 + f64x8::splat(1.174) * t227;
            let t235 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t236 = t235 * t235;
            let t237 = t236 * t236;
            let t239 = t237 * t235 * t18;
            let t240 = f64x8::splat(1.0) / t30;
            let t241 = t240 * t122;
            let t243 = f64x8::splat(1.0) / t133 / t132;
            let t245 = t239 * t241 * t243;
            let t247 = t126 * t128 * t137;
            let t251 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t122 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t230 * t138 - f64x8::splat(1.6891736332904388) * t245 * t247));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t251 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t256 = v_sigma * t27;
            let t260 = f64x8::splat(1.0) / t170;
            let t265 = t25 * t28;
            let t269 = t74 * t78;
            let t270 = param_eta * t28;
            let t271 = t270 * t33;
            let t274 = t186 * t271 / f64x8::splat(8.0) + t269 / f64x8::splat(8.0);
            let t275 = t60 * t274;
            let t278 = t274 * t84;
            let t281 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t61 * t265 * t33 + t275 * t84 / f64x8::splat(100.0) - t194 * t278 / f64x8::splat(100.0);
            let t284 = f64x8::splat(5.0) / f64x8::splat(972.0) * t26 * t74 + t44 * t256 * t56 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(640.0) * t168 * t45 * t260 * t55 + f64x8::splat(2.0) * t87 * t281;
            let t285 = t284 * t119;
            let t287 = -t274;
            let t288 = ((t96).select(f64x8::splat(0.0), t287));
            let t290 = t97 * t288;
            let t292 = t99 * t288;
            let t294 = t101 * t288;
            let t296 = t103 * t288;
            let t298 = t105 * t288;
            let t303 = ((t96).select(t287, f64x8::splat(0.0)));
            let t307 = ((t95).select(-f64x8::splat(0.667) * t288 - f64x8::splat(0.889111) * t290 - f64x8::splat(1.989259803147) * t292 + f64x8::splat(5.80518817796) * t294 - f64x8::splat(4.439990207985) * t296 + f64x8::splat(1.407173648874) * t298 - f64x8::splat(0.162300903254) * t107 * t288, -t220 * t222 * t303 * t116));
            let t310 = t151 * t285 - t94 * t307 + f64x8::splat(1.174) * t307;
            let t315 = f64x8::splat(1.0) / v_rho;
            let t316 = t315 * t122;
            let t318 = t239 * t316 * t243;
            let t319 = f64x8::splat(1.0) / t127;
            let t322 = t126 * t319 * t27 * t137;
            let t326 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t310 * t138 + f64x8::splat(0.6334401124839145) * t318 * t322));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t326;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t328 = t60 * t28;
            let t329 = t67 * t78;
            let t330 = t329 * t84;
            let t332 = t194 * t28;
            let t335 = -t328 * t330 / f64x8::splat(100.0) + t332 * t330 / f64x8::splat(100.0);
            let t336 = t87 * t335;
            let t340 = t28 * t67;
            let t341 = t340 * t78;
            let t342 = ((t96).select(f64x8::splat(0.0), t341));
            let t344 = t97 * t342;
            let t346 = t99 * t342;
            let t348 = t101 * t342;
            let t350 = t103 * t342;
            let t352 = t105 * t342;
            let t357 = ((t96).select(t341, f64x8::splat(0.0)));
            let t361 = ((t95).select(-f64x8::splat(0.667) * t342 - f64x8::splat(0.889111) * t344 - f64x8::splat(1.989259803147) * t346 + f64x8::splat(5.80518817796) * t348 - f64x8::splat(4.439990207985) * t350 + f64x8::splat(1.407173648874) * t352 - f64x8::splat(0.162300903254) * t107 * t342, -t220 * t222 * t357 * t116));
            let t364 = f64x8::splat(2.0) * t151 * t336 * t119 - t94 * t361 + f64x8::splat(1.174) * t361;
            let t369 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t364 * t138));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t369;
            acc_vtau = tvtau0;
            let t380 = f64x8::splat(1.0) / t152;
            let t381 = t380 * t122;
            let t383 = t239 * t381 * t243;
            let t388 = t148 / t149 / t89;
            let t389 = t201 * t201;
            let t390 = t389 * t119;
            let t394 = f64x8::splat(1.0) / t31 / t47;
            let t395 = t29 * t394;
            let t398 = t47 * t152;
            let t400 = f64x8::splat(1.0) / t20 / t398;
            let t401 = t400 * t55;
            let t405 = t170 * t30;
            let t406 = f64x8::splat(1.0) / t405;
            let t412 = t165 * t39 * t167;
            let t413 = t45 * t45;
            let t414 = t170 * t47;
            let t416 = f64x8::splat(1.0) / t31 / t414;
            let t419 = t28 * t55;
            let t420 = t26 * t419;
            let t423 = t198 * t198;
            let t430 = f64x8::splat(40.0) / f64x8::splat(9.0) * t65 * t154 - f64x8::splat(11.0) / f64x8::splat(9.0) * t395;
            let t432 = t182 * t185;
            let t433 = t432 * param_eta;
            let t437 = f64x8::splat(1.0) / t184 / t77;
            let t438 = t70 * t437;
            let t439 = param_eta * param_eta;
            let t440 = t438 * t439;
            let t441 = t46 * t400;
            let t446 = -t430 * t78 - f64x8::splat(2.0) / f64x8::splat(3.0) * t433 * t155 - f64x8::splat(4.0) / f64x8::splat(9.0) * t440 * t441 + f64x8::splat(11.0) / f64x8::splat(9.0) * t187 * t395;
            let t447 = t60 * t446;
            let t450 = t190 * t190;
            let t452 = t80 * t84;
            let t455 = t446 * t84;
            let t458 = t82 * t80;
            let t459 = t60 * t458;
            let t460 = t450 * t84;
            let t463 = f64x8::splat(77.0) / f64x8::splat(14580.0) * t62 * t395 + t447 * t84 / f64x8::splat(100.0) - f64x8::splat(3.0) / f64x8::splat(100.0) * t60 * t450 * t452 - t194 * t455 / f64x8::splat(100.0) + t459 * t460 / f64x8::splat(100.0);
            let t466 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t26 * t395 + f64x8::splat(19.0) / f64x8::splat(162.0) * t44 * t46 * t401 - f64x8::splat(43.0) / f64x8::splat(80.0) * t168 * t169 * t406 * t55 + f64x8::splat(27.0) / f64x8::splat(800.0) * t412 * t413 * t416 * t420 + f64x8::splat(2.0) * t423 + f64x8::splat(2.0) * t87 * t463;
            let t469 = t201 * t227;
            let t472 = -t446;
            let t473 = ((t96).select(f64x8::splat(0.0), t472));
            let t475 = t205 * t205;
            let t479 = t97 * t475;
            let t483 = t99 * t475;
            let t487 = t101 * t475;
            let t491 = t103 * t475;
            let t499 = -f64x8::splat(0.667) * t473 - f64x8::splat(0.889111) * t475 - f64x8::splat(0.889111) * t97 * t473 - f64x8::splat(3.978519606294) * t479 - f64x8::splat(1.989259803147) * t99 * t473 + f64x8::splat(17.41556453388) * t483 + f64x8::splat(5.80518817796) * t101 * t473 - f64x8::splat(17.75996083194) * t487 - f64x8::splat(4.439990207985) * t103 * t473 + f64x8::splat(7.03586824437) * t491 + f64x8::splat(1.407173648874) * t105 * t473 - f64x8::splat(0.973805419524) * t105 * t475 - f64x8::splat(0.162300903254) * t107 * t473;
            let t500 = t221 * t113;
            let t501 = f64x8::splat(1.0) / t500;
            let t502 = t223 * t223;
            let t507 = ((t96).select(t472, f64x8::splat(0.0)));
            let t511 = param_c2 * param_c2;
            let t512 = param_d * t511;
            let t513 = t221 * t221;
            let t514 = f64x8::splat(1.0) / t513;
            let t519 = ((t95).select(t499, -t220 * t222 * t507 * t116 - f64x8::splat(2.0) * t220 * t501 * t502 * t116 - t512 * t514 * t502 * t116));
            let t522 = -f64x8::splat(2.0) * t388 * t390 + t151 * t466 * t119 - f64x8::splat(2.0) * t151 * t469 - t94 * t519 + f64x8::splat(1.174) * t519;
            let t529 = t239 * t240 * t230 * t243;
            let t533 = f64x8::splat(1.0) / t20 / t47;
            let t534 = t533 * t122;
            let t537 = f64x8::splat(1.0) / t133 / t35 / f64x8::splat(6.0);
            let t539 = t239 * t534 * t537;
            let t541 = t26 * t29 * t137;
            let t544 = t4 * t18;
            let t545 = f64x8::splat(1.0) / t20;
            let t547 = t544 * t545 * t122;
            let t549 = t265 * t137;
            let t550 = t319 * t21 * t549;
            let t554 = ((t3).select(f64x8::splat(0.0), t19 * t67 * t122 * t138 / f64x8::splat(12.0) - t19 * t143 * t230 * t138 / f64x8::splat(4.0) + f64x8::splat(2.8152893888173978) * t383 * t247 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t522 * t138 - f64x8::splat(3.3783472665808776) * t529 * t247 - f64x8::splat(20.270083599485265) * t539 * t541 + f64x8::splat(27.496264583922507) * t547 * t550));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t554 + f64x8::splat(4.0) * t251;
            acc_v2rho2 = tv2rho20;
            let t561 = t285 * t201;
            let t564 = t28 * t154;
            let t574 = t170 * t152;
            let t576 = f64x8::splat(1.0) / t31 / t574;
            let t586 = t564 * t78;
            let t588 = t27 * t160;
            let t589 = t185 * param_eta;
            let t590 = t589 * v_sigma;
            let t591 = t588 * t590;
            let t595 = t588 * v_sigma;
            let t598 = t270 * t154;
            let t601 = -t586 / f64x8::splat(3.0) + t591 / f64x8::splat(12.0) + t432 * t271 / f64x8::splat(8.0) + t440 * t595 / f64x8::splat(6.0) - t186 * t598 / f64x8::splat(3.0);
            let t602 = t60 * t601;
            let t605 = t80 * t190;
            let t606 = t605 * t84;
            let t609 = t601 * t84;
            let t612 = t274 * t190;
            let t613 = t612 * t84;
            let t616 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t61 * t265 * t154 + t602 * t84 / f64x8::splat(100.0) - f64x8::splat(3.0) / f64x8::splat(100.0) * t275 * t606 - t194 * t609 / f64x8::splat(100.0) + t459 * t613 / f64x8::splat(100.0);
            let t619 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t564 - t44 * t256 * t161 / f64x8::splat(27.0) + f64x8::splat(3.0) / f64x8::splat(16.0) * t168 * t45 * t172 * t55 - f64x8::splat(81.0) / f64x8::splat(6400.0) * t412 * t169 * t576 * t420 + f64x8::splat(2.0) * t198 * t281 + f64x8::splat(2.0) * t87 * t616;
            let t620 = t619 * t119;
            let t622 = t284 * t227;
            let t624 = t201 * t307;
            let t626 = -t601;
            let t627 = ((t96).select(f64x8::splat(0.0), t626));
            let t629 = t205 * t288;
            let t631 = t97 * t627;
            let t635 = t99 * t627;
            let t639 = t101 * t627;
            let t643 = t103 * t627;
            let t647 = t105 * t627;
            let t653 = -f64x8::splat(0.667) * t627 - f64x8::splat(0.889111) * t629 - f64x8::splat(0.889111) * t631 - f64x8::splat(3.978519606294) * t290 * t205 - f64x8::splat(1.989259803147) * t635 + f64x8::splat(17.41556453388) * t292 * t205 + f64x8::splat(5.80518817796) * t639 - f64x8::splat(17.75996083194) * t294 * t205 - f64x8::splat(4.439990207985) * t643 + f64x8::splat(7.03586824437) * t296 * t205 + f64x8::splat(1.407173648874) * t647 - f64x8::splat(0.973805419524) * t298 * t205 - f64x8::splat(0.162300903254) * t107 * t627;
            let t654 = t220 * t501;
            let t655 = t303 * t116;
            let t656 = t655 * t223;
            let t659 = ((t96).select(t626, f64x8::splat(0.0)));
            let t663 = t512 * t514;
            let t666 = ((t95).select(t653, -t220 * t222 * t659 * t116 - f64x8::splat(2.0) * t654 * t656 - t663 * t656));
            let t669 = -f64x8::splat(2.0) * t388 * t561 + t151 * t620 - t151 * t622 - t151 * t624 - t94 * t666 + f64x8::splat(1.174) * t666;
            let t676 = t239 * t240 * t310 * t243;
            let t683 = t239 * t315 * t230 * t243;
            let t687 = f64x8::splat(1.0) / t20 / t152;
            let t688 = t687 * t122;
            let t691 = t537 * t21 * t549;
            let t695 = t544 * t31 * t122;
            let t696 = t127 * v_sigma;
            let t697 = f64x8::splat(1.0) / t696;
            let t699 = t697 * t21 * t549;
            let t703 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t310 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t669 * t138 - f64x8::splat(1.6891736332904388) * t676 * t247 - f64x8::splat(0.6334401124839145) * t245 * t322 + f64x8::splat(0.6334401124839145) * t683 * t322 + f64x8::splat(7.601281349806975) * t239 * t688 * t691 - f64x8::splat(10.311099218970941) * t695 * t699));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t703 + f64x8::splat(2.0) * t326;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t710 = t388 * t87;
            let t711 = t335 * t119;
            let t712 = t711 * t201;
            let t715 = t198 * t335;
            let t719 = t33 * t78;
            let t720 = t719 * t84;
            let t723 = t60 * t27;
            let t724 = t723 * t50;
            let t725 = t185 * t84;
            let t726 = t725 * t73;
            let t729 = t328 * t67;
            let t730 = t78 * t80;
            let t731 = t730 * t195;
            let t736 = t27 * t50;
            let t740 = t459 * t28;
            let t744 = t328 * t720 / f64x8::splat(60.0) - t724 * t726 / f64x8::splat(150.0) + f64x8::splat(3.0) / f64x8::splat(100.0) * t729 * t731 - t332 * t720 / f64x8::splat(60.0) + t194 * t736 * t726 / f64x8::splat(150.0) - t740 * t329 * t195 / f64x8::splat(100.0);
            let t745 = t87 * t744;
            let t752 = t201 * t361;
            let t757 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t269 + f64x8::splat(2.0) / f64x8::splat(3.0) * t736 * t590;
            let t758 = ((t96).select(f64x8::splat(0.0), t757));
            let t760 = t205 * t342;
            let t762 = t97 * t758;
            let t766 = t99 * t758;
            let t770 = t101 * t758;
            let t774 = t103 * t758;
            let t778 = t105 * t758;
            let t784 = -f64x8::splat(0.667) * t758 - f64x8::splat(0.889111) * t760 - f64x8::splat(0.889111) * t762 - f64x8::splat(3.978519606294) * t344 * t205 - f64x8::splat(1.989259803147) * t766 + f64x8::splat(17.41556453388) * t346 * t205 + f64x8::splat(5.80518817796) * t770 - f64x8::splat(17.75996083194) * t348 * t205 - f64x8::splat(4.439990207985) * t774 + f64x8::splat(7.03586824437) * t350 * t205 + f64x8::splat(1.407173648874) * t778 - f64x8::splat(0.973805419524) * t352 * t205 - f64x8::splat(0.162300903254) * t107 * t758;
            let t785 = t357 * t116;
            let t786 = t785 * t223;
            let t789 = ((t96).select(t757, f64x8::splat(0.0)));
            let t795 = ((t95).select(t784, -t220 * t222 * t789 * t116 - f64x8::splat(2.0) * t654 * t786 - t663 * t786));
            let t798 = -f64x8::splat(4.0) * t710 * t712 + f64x8::splat(2.0) * t151 * t715 * t119 + f64x8::splat(2.0) * t151 * t745 * t119 - f64x8::splat(2.0) * t151 * t336 * t227 - t151 * t752 - t94 * t795 + f64x8::splat(1.174) * t795;
            let t805 = t239 * t240 * t364 * t243;
            let t809 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t364 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t798 * t138 - f64x8::splat(1.6891736332904388) * t805 * t247));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t809 + f64x8::splat(2.0) * t369;
            acc_v2rhotau = tv2rhotau0;
            let t812 = t284 * t284;
            let t813 = t812 * t119;
            let t824 = f64x8::splat(1.0) / t31 / t405;
            let t829 = t281 * t281;
            let t831 = t736 * t589;
            let t832 = t439 * t27;
            let t833 = t832 * t50;
            let t836 = -t438 * t833 / f64x8::splat(16.0) - t831 / f64x8::splat(16.0);
            let t837 = t60 * t836;
            let t840 = t274 * t274;
            let t841 = t60 * t840;
            let t844 = t836 * t84;
            let t847 = t840 * t84;
            let t850 = t837 * t84 / f64x8::splat(100.0) - f64x8::splat(3.0) / f64x8::splat(100.0) * t841 * t452 - t194 * t844 / f64x8::splat(100.0) + t459 * t847 / f64x8::splat(100.0);
            let t853 = t44 * t736 * t55 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(160.0) * t168 * v_sigma * t260 * t55 + f64x8::splat(243.0) / f64x8::splat(51200.0) * t412 * t45 * t824 * t420 + f64x8::splat(2.0) * t829 + f64x8::splat(2.0) * t87 * t850;
            let t854 = t853 * t119;
            let t856 = t284 * t307;
            let t859 = -t836;
            let t860 = ((t96).select(f64x8::splat(0.0), t859));
            let t862 = t288 * t288;
            let t864 = t97 * t860;
            let t866 = t97 * t862;
            let t868 = t99 * t860;
            let t870 = t99 * t862;
            let t872 = t101 * t860;
            let t874 = t101 * t862;
            let t876 = t103 * t860;
            let t878 = t103 * t862;
            let t880 = t105 * t860;
            let t886 = -f64x8::splat(0.667) * t860 - f64x8::splat(0.889111) * t862 - f64x8::splat(0.889111) * t864 - f64x8::splat(3.978519606294) * t866 - f64x8::splat(1.989259803147) * t868 + f64x8::splat(17.41556453388) * t870 + f64x8::splat(5.80518817796) * t872 - f64x8::splat(17.75996083194) * t874 - f64x8::splat(4.439990207985) * t876 + f64x8::splat(7.03586824437) * t878 + f64x8::splat(1.407173648874) * t880 - f64x8::splat(0.973805419524) * t105 * t862 - f64x8::splat(0.162300903254) * t107 * t860;
            let t887 = t303 * t303;
            let t892 = ((t96).select(t859, f64x8::splat(0.0)));
            let t900 = ((t95).select(t886, -t220 * t222 * t892 * t116 - f64x8::splat(2.0) * t220 * t501 * t887 * t116 - t512 * t514 * t887 * t116));
            let t903 = -f64x8::splat(2.0) * t388 * t813 + t151 * t854 - f64x8::splat(2.0) * t151 * t856 - t94 * t900 + f64x8::splat(1.174) * t900;
            let t910 = t239 * t315 * t310 * t243;
            let t914 = f64x8::splat(1.0) / t20 / t30;
            let t915 = t914 * t122;
            let t917 = t239 * t915 * t537;
            let t918 = f64x8::splat(1.0) / v_sigma;
            let t921 = t26 * t918 * t28 * t137;
            let t926 = t126 * t697 * t27 * t137;
            let t930 = t544 * t66 * t122;
            let t932 = f64x8::splat(1.0) / t127 / t45;
            let t934 = t932 * t21 * t549;
            let t938 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t903 * t138 + f64x8::splat(1.266880224967829) * t910 * t322 - f64x8::splat(2.8504805061776155) * t917 * t921 - f64x8::splat(0.31672005624195726) * t318 * t926 + f64x8::splat(3.8666622071141026) * t930 * t934));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t938;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t940 = t711 * t284;
            let t943 = t281 * t335;
            let t947 = t723 * t533;
            let t948 = t725 * param_eta;
            let t951 = t730 * t278;
            let t954 = t194 * t27;
            let t955 = t533 * t185;
            let t956 = t84 * param_eta;
            let t963 = t947 * t948 / f64x8::splat(400.0) + f64x8::splat(3.0) / f64x8::splat(100.0) * t729 * t951 - t954 * t955 * t956 / f64x8::splat(400.0) - t740 * t329 * t278 / f64x8::splat(100.0);
            let t964 = t87 * t963;
            let t971 = t284 * t361;
            let t973 = t27 * t533;
            let t975 = t973 * t589 / f64x8::splat(4.0);
            let t976 = ((t96).select(f64x8::splat(0.0), -t975));
            let t978 = t288 * t342;
            let t980 = t97 * t976;
            let t984 = t99 * t976;
            let t988 = t101 * t976;
            let t992 = t103 * t976;
            let t996 = t105 * t976;
            let t1002 = -f64x8::splat(0.667) * t976 - f64x8::splat(0.889111) * t978 - f64x8::splat(0.889111) * t980 - f64x8::splat(3.978519606294) * t344 * t288 - f64x8::splat(1.989259803147) * t984 + f64x8::splat(17.41556453388) * t346 * t288 + f64x8::splat(5.80518817796) * t988 - f64x8::splat(17.75996083194) * t348 * t288 - f64x8::splat(4.439990207985) * t992 + f64x8::splat(7.03586824437) * t350 * t288 + f64x8::splat(1.407173648874) * t996 - f64x8::splat(0.973805419524) * t352 * t288 - f64x8::splat(0.162300903254) * t107 * t976;
            let t1003 = t785 * t303;
            let t1006 = ((t96).select(-t975, f64x8::splat(0.0)));
            let t1012 = ((t95).select(t1002, -t220 * t222 * t1006 * t116 - f64x8::splat(2.0) * t654 * t1003 - t663 * t1003));
            let t1015 = -f64x8::splat(4.0) * t710 * t940 + f64x8::splat(2.0) * t151 * t943 * t119 + f64x8::splat(2.0) * t151 * t964 * t119 - f64x8::splat(2.0) * t151 * t336 * t307 - t151 * t971 - t94 * t1012 + f64x8::splat(1.174) * t1012;
            let t1022 = t239 * t315 * t364 * t243;
            let t1026 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t1015 * t138 + f64x8::splat(0.6334401124839145) * t1022 * t322));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t1026;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1028 = t335 * t335;
            let t1029 = t88 * t1028;
            let t1033 = t1028 * t119;
            let t1036 = t723 * t687;
            let t1038 = t185 * t80 * t84;
            let t1041 = t459 * t27;
            let t1042 = t687 * t185;
            let t1046 = -f64x8::splat(3.0) / f64x8::splat(50.0) * t1036 * t1038 + t1041 * t1042 * t84 / f64x8::splat(50.0);
            let t1047 = t87 * t1046;
            let t1054 = ((t96).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t1055 = f64x8::splat(0.667) * t1054;
            let t1056 = t342 * t342;
            let t1058 = t97 * t1054;
            let t1059 = f64x8::splat(0.889111) * t1058;
            let t1060 = t97 * t1056;
            let t1062 = t99 * t1054;
            let t1063 = f64x8::splat(1.989259803147) * t1062;
            let t1064 = t99 * t1056;
            let t1066 = t101 * t1054;
            let t1067 = f64x8::splat(5.80518817796) * t1066;
            let t1068 = t101 * t1056;
            let t1070 = t103 * t1054;
            let t1071 = f64x8::splat(4.439990207985) * t1070;
            let t1072 = t103 * t1056;
            let t1074 = t105 * t1054;
            let t1075 = f64x8::splat(1.407173648874) * t1074;
            let t1079 = f64x8::splat(0.162300903254) * t107 * t1054;
            let t1080 = -t1055 - f64x8::splat(0.889111) * t1056 - t1059 - f64x8::splat(3.978519606294) * t1060 - t1063 + f64x8::splat(17.41556453388) * t1064 + t1067 - f64x8::splat(17.75996083194) * t1068 - t1071 + f64x8::splat(7.03586824437) * t1072 + t1075 - f64x8::splat(0.973805419524) * t105 * t1056 - t1079;
            let t1081 = t357 * t357;
            let t1088 = t220 * t222 * t1054 * t116;
            let t1093 = ((t95).select(t1080, -f64x8::splat(2.0) * t220 * t501 * t1081 * t116 - t512 * t514 * t1081 * t116 - t1088));
            let t1096 = -f64x8::splat(8.0) * t388 * t1029 * t119 + f64x8::splat(2.0) * t151 * t1033 + f64x8::splat(2.0) * t151 * t1047 * t119 - f64x8::splat(4.0) * t151 * t336 * t361 - t94 * t1093 + f64x8::splat(1.174) * t1093;
            let t1101 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t1096 * t138));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t1101;
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
