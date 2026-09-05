//! GGA_X_LG93 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`
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
pub fn gga_x_lg93_lxc_unpol(
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
            let t18 = t6 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t19 * t19;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = t20 * t20;
            let t38 = f64x8::splat(1.0) / t22 / t21;
            let t39 = t36 * t38;
            let t40 = v_sigma * v_sigma;
            let t41 = t40 * t26;
            let t42 = t29 * t29;
            let t43 = t42 * v_rho;
            let t45 = f64x8::splat(1.0) / t19 / t43;
            let t49 = t40 * v_sigma;
            let t50 = t42 * t42;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = t21 * t21;
            let t57 = t20 / t23 / t54;
            let t58 = t40 * t40;
            let t59 = t58 * t27;
            let t60 = t50 * t29;
            let t62 = f64x8::splat(1.0) / t30 / t60;
            let t69 = t36 / t22 / t54 / t21;
            let t70 = t58 * v_sigma;
            let t71 = t70 * t26;
            let t72 = t50 * t43;
            let t74 = f64x8::splat(1.0) / t19 / t72;
            let t78 = t58 * t40;
            let t79 = t50 * t50;
            let t80 = f64x8::splat(1.0) / t79;
            let t83 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t34 + f64x8::splat(0.1034375) * t39 * t41 * t45 + f64x8::splat(0.0003995356322973242) * t49 * t51 + f64x8::splat(0.0008766637731481481) * t57 * t59 * t62 + f64x8::splat(0.009464819637345679) * t69 * t71 * t74 + f64x8::splat(1.7770905884280507e-08) * t78 * t80;
            let t84 = (simd::pow(t83, f64x8::splat(0.024974)));
            let t87 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t34;
            let t88 = f64x8::splat(1.0) / t87;
            let t92 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t84 * t88));
            let tzk0 = f64x8::splat(2.0) * t92;
            acc_zk = tzk0;
            let t93 = f64x8::splat(1.0) / t30;
            let t98 = (simd::pow(t83, -f64x8::splat(0.975026)));
            let t99 = t19 * t98;
            let t100 = t29 * v_rho;
            let t102 = f64x8::splat(1.0) / t30 / t100;
            let t106 = t42 * t29;
            let t108 = f64x8::splat(1.0) / t19 / t106;
            let t112 = t50 * v_rho;
            let t113 = f64x8::splat(1.0) / t112;
            let t116 = t50 * t100;
            let t118 = f64x8::splat(1.0) / t30 / t116;
            let t122 = t50 * t106;
            let t124 = f64x8::splat(1.0) / t19 / t122;
            let t128 = t79 * v_rho;
            let t129 = f64x8::splat(1.0) / t128;
            let t132 = -f64x8::splat(0.5490154649724602) * t25 * t28 * t102 - f64x8::splat(0.5516666666666666) * t39 * t41 * t108 - f64x8::splat(0.0031962850583785937) * t49 * t113 - f64x8::splat(0.009351080246913581) * t57 * t59 * t118 - f64x8::splat(0.12619759516460904) * t69 * t71 * t124 - f64x8::splat(2.843344941484881e-07) * t78 * t129;
            let t133 = t88 * t132;
            let t137 = t3 * t17;
            let t139 = f64x8::splat(1.0) / t19 / t100;
            let t141 = t137 * t139 * t84;
            let t142 = t87 * t87;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t143 * t20;
            let t146 = t24 * v_sigma * t27;
            let t147 = t144 * t146;
            let t151 = ((t2).select(f64x8::splat(0.0), -t18 * t93 * t84 * t88 / f64x8::splat(8.0) - f64x8::splat(0.00936525) * t18 * t99 * t133 - f64x8::splat(2.8449335968970655e-10) * t141 * t147));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t151 + f64x8::splat(2.0) * t92;
            acc_vrho = tvrho0;
            let t157 = v_sigma * t26;
            let t163 = t49 * t27;
            let t167 = t58 * t26;
            let t173 = f64x8::splat(0.2058807993646726) * t25 * t27 * t32 + f64x8::splat(0.206875) * t39 * t157 * t45 + f64x8::splat(0.0011986068968919726) * t40 * t51 + f64x8::splat(0.0035066550925925925) * t57 * t163 * t62 + f64x8::splat(0.04732409818672839) * t69 * t167 * t74 + f64x8::splat(1.0662543530568304e-07) * t70 * t80;
            let t174 = t88 * t173;
            let t179 = f64x8::splat(1.0) / t19 / t29;
            let t182 = t24 * t27;
            let t183 = t144 * t182;
            let t187 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.00936525) * t18 * t99 * t174 + f64x8::splat(1.0668500988363994e-10) * t137 * t179 * t84 * t183));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t187;
            acc_vsigma = tvsigma0;
            let t191 = f64x8::splat(1.0) / t30 / v_rho;
            let t196 = t93 * t98;
            let t201 = f64x8::splat(1.0) / t19 / t42;
            let t203 = t137 * t201 * t84;
            let t206 = (simd::pow(t83, -f64x8::splat(1.975026)));
            let t207 = t19 * t206;
            let t208 = t132 * t132;
            let t209 = t88 * t208;
            let t213 = t139 * t98;
            let t215 = t137 * t213 * t143;
            let t216 = t132 * t20;
            let t217 = t216 * t146;
            let t221 = f64x8::splat(1.0) / t30 / t42;
            let t225 = t42 * t100;
            let t227 = f64x8::splat(1.0) / t19 / t225;
            let t231 = f64x8::splat(1.0) / t60;
            let t234 = t50 * t42;
            let t236 = f64x8::splat(1.0) / t30 / t234;
            let t242 = f64x8::splat(1.0) / t19 / t50 / t225;
            let t247 = f64x8::splat(1.0) / t79 / t29;
            let t250 = f64x8::splat(2.013056704899021) * t25 * t28 * t221 + f64x8::splat(3.493888888888889) * t39 * t41 * t227 + f64x8::splat(0.028766565525407344) * t49 * t231 + f64x8::splat(0.10909593621399177) * t57 * t59 * t236 + f64x8::splat(1.8088321973593964) * t69 * t71 * t242 + f64x8::splat(4.833686400524298e-06) * t78 * t247;
            let t251 = t88 * t250;
            let t255 = f64x8::splat(1.0) / t225;
            let t257 = t137 * t255 * t84;
            let t259 = f64x8::splat(1.0) / t142 / t87;
            let t260 = t259 * t36;
            let t262 = t38 * t40 * t26;
            let t263 = t260 * t262;
            let t267 = ((t2).select(f64x8::splat(0.0), t18 * t191 * t84 * t88 / f64x8::splat(12.0) - f64x8::splat(0.0062435) * t18 * t196 * t133 + f64x8::splat(8.534800790691196e-10) * t203 * t147 + f64x8::splat(0.0091313622465) * t18 * t207 * t209 - f64x8::splat(1.4209874329781462e-11) * t215 * t217 - f64x8::splat(0.00936525) * t18 * t99 * t251 - f64x8::splat(1.2644149319542513e-18) * t257 * t263));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t267 + f64x8::splat(4.0) * t151;
            acc_v2rho2 = tv2rho20;
            let t274 = t6 * t17 * t19;
            let t275 = t206 * t88;
            let t276 = t173 * t132;
            let t277 = t275 * t276;
            let t280 = t173 * t20;
            let t281 = t280 * t146;
            let t300 = -f64x8::splat(0.5490154649724602) * t25 * t27 * t102 - f64x8::splat(1.1033333333333333) * t39 * t157 * t108 - f64x8::splat(0.009588855175135781) * t40 * t113 - f64x8::splat(0.037404320987654324) * t57 * t163 * t118 - f64x8::splat(0.6309879758230452) * t69 * t167 * t124 - f64x8::splat(1.7060069648909286e-06) * t70 * t129;
            let t301 = t88 * t300;
            let t308 = t137 * t179 * t98;
            let t309 = t182 * t132;
            let t310 = t144 * t309;
            let t313 = f64x8::splat(1.0) / t106;
            let t315 = t137 * t313 * t84;
            let t316 = t38 * t26;
            let t317 = t316 * v_sigma;
            let t318 = t260 * t317;
            let t322 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.00312175) * t18 * t196 * t174 + f64x8::splat(0.0091313622465) * t274 * t277 - f64x8::splat(7.104937164890731e-12) * t215 * t281 - f64x8::splat(0.00936525) * t18 * t99 * t301 - f64x8::splat(2.4893168972849323e-10) * t141 * t183 + f64x8::splat(2.664351436834024e-12) * t308 * t310 + f64x8::splat(4.741555994828442e-19) * t315 * t318));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t322 + f64x8::splat(2.0) * t187;
            acc_v2rhosigma = tv2rhosigma0;
            let t325 = t173 * t173;
            let t326 = t88 * t325;
            let t331 = t25 * t27;
            let t332 = t143 * t173 * t331;
            let t340 = t40 * t27;
            let t344 = t49 * t26;
            let t350 = f64x8::splat(0.206875) * t39 * t26 * t45 + f64x8::splat(0.0023972137937839453) * v_sigma * t51 + f64x8::splat(0.010519965277777777) * t57 * t340 * t62 + f64x8::splat(0.18929639274691357) * t69 * t344 * t74 + f64x8::splat(5.331271765284152e-07) * t58 * t80;
            let t351 = t88 * t350;
            let t355 = f64x8::splat(1.0) / t43;
            let t358 = t260 * t316;
            let t362 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.0091313622465) * t18 * t207 * t326 + f64x8::splat(5.328702873668048e-12) * t308 * t332 - f64x8::splat(0.00936525) * t18 * t99 * t351 - f64x8::splat(1.7780834980606658e-19) * t137 * t355 * t84 * t358));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t362;
            acc_v2sigma2 = tv2sigma20;
            let t365 = t137 * t62;
            let t366 = t142 * t142;
            let t367 = f64x8::splat(1.0) / t366;
            let t368 = t84 * t367;
            let t369 = t368 * t49;
            let t372 = t201 * t98;
            let t374 = t137 * t372 * t143;
            let t377 = t139 * t206;
            let t379 = t137 * t377 * t143;
            let t381 = t208 * t20 * t146;
            let t385 = t250 * t20 * t146;
            let t390 = t137 * t255 * t98 * t259;
            let t392 = t132 * t36 * t262;
            let t399 = f64x8::splat(1.0) / t30 / t43;
            let t404 = f64x8::splat(1.0) / t19 / t50;
            let t408 = f64x8::splat(1.0) / t116;
            let t412 = f64x8::splat(1.0) / t30 / t72;
            let t417 = f64x8::splat(1.0) / t19 / t79;
            let t422 = f64x8::splat(1.0) / t79 / t100;
            let t425 = -f64x8::splat(9.394264622862098) * t25 * t28 * t399 - f64x8::splat(25.62185185185185) * t39 * t41 * t404 - f64x8::splat(0.28766565525407345) * t49 * t408 - f64x8::splat(1.3818818587105623) * t57 * t59 * t412 - f64x8::splat(27.735427026177412) * t69 * t71 * t417 - f64x8::splat(8.700635520943737e-05) * t78 * t422;
            let t426 = t88 * t425;
            let t434 = t191 * t98;
            let t438 = t93 * t206;
            let t442 = (simd::pow(t83, -f64x8::splat(2.975026)));
            let t443 = t19 * t442;
            let t444 = t208 * t132;
            let t445 = t88 * t444;
            let t450 = t275 * t132 * t250;
            let t454 = t137 * t45 * t84;
            let t458 = t137 * t51 * t84;
            let t461 = -f64x8::splat(5.192184501600098e-28) * t365 * t369 + f64x8::splat(6.394443448401658e-11) * t374 * t217 + f64x8::splat(2.078249539240425e-11) * t379 * t381 - f64x8::splat(2.1314811494672192e-11) * t215 * t385 - f64x8::splat(9.473249553187641e-20) * t390 * t392 - f64x8::splat(0.00936525) * t18 * t196 * t251 - f64x8::splat(0.00936525) * t18 * t99 * t426 - f64x8::splat(5.0) / f64x8::splat(36.0) * t18 * t32 * t84 * t88 + f64x8::splat(0.0062435) * t18 * t434 * t133 + f64x8::splat(0.0091313622465) * t18 * t438 * t209 - f64x8::splat(0.01803467785225591) * t18 * t443 * t445 + f64x8::splat(0.0273940867395) * t274 * t450 - f64x8::splat(3.6351929293684726e-09) * t454 * t147 + f64x8::splat(1.2644149319542512e-17) * t458 * t263;
            let t462 = ((t2).select(f64x8::splat(0.0), t461));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t462 + f64x8::splat(6.0) * t267;
            acc_v3rho3 = tv3rho30;
            let t466 = t300 * t132;
            let t467 = t275 * t466;
            let t470 = t173 * t250;
            let t471 = t275 * t470;
            let t476 = t137 / t30 / t112;
            let t477 = t368 * t40;
            let t480 = t173 * t36;
            let t481 = t480 * t262;
            let t484 = t179 * t206;
            let t485 = t137 * t484;
            let t486 = t182 * t208;
            let t487 = t144 * t486;
            let t490 = t313 * t98;
            let t492 = t137 * t490 * t259;
            let t494 = t26 * t132 * v_sigma;
            let t495 = t39 * t494;
            let t500 = t300 * t20;
            let t501 = t500 * t146;
            let t523 = f64x8::splat(2.013056704899021) * t25 * t27 * t221 + f64x8::splat(6.987777777777778) * t39 * t157 * t227 + f64x8::splat(0.08629969657622202) * t40 * t231 + f64x8::splat(0.4363837448559671) * t57 * t163 * t236 + f64x8::splat(9.044160986796982) * t69 * t167 * t242 + f64x8::splat(2.900211840314579e-05) * t70 * t247;
            let t524 = t88 * t523;
            let t529 = t276 * t20 * t146;
            let t536 = t6 * t17 * t93;
            let t539 = t442 * t88;
            let t540 = t173 * t208;
            let t541 = t539 * t540;
            let t546 = t137 * t213;
            let t551 = t182 * t250;
            let t552 = t144 * t551;
            let t555 = f64x8::splat(0.018262724493) * t274 * t467 + f64x8::splat(0.0091313622465) * t274 * t471 + f64x8::splat(1.9470691881000366e-28) * t476 * t477 - f64x8::splat(3.1577498510625473e-20) * t390 * t481 - f64x8::splat(2.5978119240505313e-12) * t485 * t487 + f64x8::splat(2.3683123882969104e-20) * t492 * t495 + f64x8::splat(2.1314811494672192e-11) * t374 * t281 - f64x8::splat(1.4209874329781462e-11) * t215 * t501 - f64x8::splat(0.0062435) * t18 * t196 * t301 - f64x8::splat(0.00936525) * t18 * t99 * t524 + f64x8::splat(1.38549969282695e-11) * t379 * t529 + f64x8::splat(0.0020811666666666665) * t18 * t434 * t174 + f64x8::splat(0.006087574831) * t536 * t277 - f64x8::splat(0.01803467785225591) * t274 * t541 + f64x8::splat(8.297722990949774e-10) * t203 * t183 - f64x8::splat(1.2433640038558779e-11) * t546 * t310 - f64x8::splat(3.951296662357035e-18) * t257 * t318 + f64x8::splat(2.664351436834024e-12) * t308 * t552;
            let t556 = ((t2).select(f64x8::splat(0.0), t555));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t556 + f64x8::splat(4.0) * t322;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t562 = t325 * t132;
            let t563 = t539 * t562;
            let t566 = t325 * t20;
            let t567 = t566 * t146;
            let t571 = t275 * t173 * t300;
            let t577 = t137 * t484 * t143;
            let t578 = t280 * t309;
            let t581 = t480 * t317;
            let t585 = t143 * t300 * t331;
            let t591 = t350 * t132;
            let t592 = t275 * t591;
            let t595 = t350 * t20;
            let t596 = t595 * t146;
            let t612 = -f64x8::splat(1.1033333333333333) * t39 * t26 * t108 - f64x8::splat(0.019177710350271562) * v_sigma * t113 - f64x8::splat(0.11221296296296296) * t57 * t340 * t118 - f64x8::splat(2.523951903292181) * t69 * t344 * t124 - f64x8::splat(8.530034824454643e-06) * t58 * t129;
            let t613 = t88 * t612;
            let t620 = t137 * t355 * t98;
            let t621 = t316 * t132;
            let t622 = t260 * t621;
            let t626 = f64x8::splat(1.0) / t30 / t50;
            let t627 = t137 * t626;
            let t628 = t368 * v_sigma;
            let t631 = f64x8::splat(0.0030437874155) * t18 * t438 * t326 - f64x8::splat(0.01803467785225591) * t274 * t563 + f64x8::splat(6.92749846413475e-12) * t379 * t567 + f64x8::splat(0.018262724493) * t274 * t571 - f64x8::splat(1.2433640038558779e-11) * t546 * t332 - f64x8::splat(5.1956238481010625e-12) * t577 * t578 + f64x8::splat(2.3683123882969104e-20) * t492 * t581 + f64x8::splat(5.328702873668048e-12) * t308 * t585 - f64x8::splat(0.00312175) * t18 * t196 * t351 + f64x8::splat(0.0091313622465) * t274 * t592 - f64x8::splat(7.104937164890731e-12) * t215 * t596 - f64x8::splat(0.00936525) * t18 * t99 * t613 + f64x8::splat(8.890417490303329e-19) * t315 * t358 - f64x8::splat(4.440585728056707e-21) * t620 * t622 - f64x8::splat(7.301509455375138e-29) * t627 * t628;
            let t632 = ((t2).select(f64x8::splat(0.0), t631));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t632 + f64x8::splat(2.0) * t362;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t635 = t325 * t173;
            let t636 = t88 * t635;
            let t641 = t143 * t325 * t331;
            let t644 = t173 * t350;
            let t645 = t275 * t644;
            let t649 = t39 * t26;
            let t650 = t259 * t173 * t649;
            let t654 = t143 * t350 * t331;
            let t666 = f64x8::splat(0.0023972137937839453) * t51 + f64x8::splat(0.021039930555555555) * t57 * t28 * t62 + f64x8::splat(0.5678891782407407) * t69 * t41 * t74 + f64x8::splat(2.1325087061136608e-06) * t49 * t80;
            let t667 = t88 * t666;
            let t672 = f64x8::splat(1.0) / t30 / t225;
            let t678 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.01803467785225591) * t18 * t443 * t636 - f64x8::splat(7.793435772151594e-12) * t485 * t641 + f64x8::splat(0.0273940867395) * t274 * t645 - f64x8::splat(1.332175718417012e-20) * t620 * t650 + f64x8::splat(7.993054310502073e-12) * t308 * t654 - f64x8::splat(0.00936525) * t18 * t99 * t667 + f64x8::splat(2.7380660457656764e-29) * t137 * t672 * t84 * t367));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t678;
            acc_v3sigma3 = tv3sigma30;
            let t698 = t98 * t367;
            let t738 = t32 * t98;
            let t742 = t191 * t206;
            let t746 = t93 * t442;
            let t750 = (simd::pow(t83, -f64x8::splat(3.975026)));
            let t751 = t19 * t750;
            let t752 = t208 * t208;
            let t757 = t250 * t250;
            let t762 = f64x8::splat(1.0730514636640203e-26) * t137 * t118 * t369 + f64x8::splat(10.0) / f64x8::splat(27.0) * t18 * t102 * t84 * t88 - f64x8::splat(0.10820806711353545) * t274 * t539 * t208 * t250 + f64x8::splat(0.036525448986) * t536 * t450 + f64x8::splat(0.036525448986) * t274 * t275 * t425 * t132 - f64x8::splat(5.186784629718434e-29) * t365 * t698 * t49 * t132 - f64x8::splat(0.012487) * t18 * t196 * t426 - f64x8::splat(0.00936525) * t18 * t99 * t88 * (f64x8::splat(53.23416619621855) * t25 * t28 / t30 / t106 + f64x8::splat(213.51543209876544) * t39 * t41 / t19 / t112 + f64x8::splat(3.164322207794808) * t49 / t234 + f64x8::splat(18.88571873571102) * t57 * t59 / t30 / t122 + f64x8::splat(453.0119747608977) * t69 * t71 / t19 / t128 + f64x8::splat(0.0016531207489793099) * t78 / t79 / t42) - f64x8::splat(0.013874444444444444) * t18 * t738 * t133 - f64x8::splat(0.012175149662) * t18 * t742 * t209 - f64x8::splat(0.02404623713634121) * t18 * t746 * t445 + f64x8::splat(0.05365363551208549) * t18 * t751 * t88 * t752 + f64x8::splat(0.0273940867395) * t18 * t207 * t88 * t757;
            let t769 = f64x8::splat(1.0) / t366 / t87;
            let t789 = t137 * t255 * t206 * t259;
            let t806 = t137 * t51 * t98 * t259;
            let t811 = t137 * t45 * t98 * t143;
            let t816 = t137 * t201 * t206 * t143;
            let t821 = t137 * t139 * t442 * t143;
            let t826 = f64x8::splat(0.012487) * t18 * t434 * t251 - f64x8::splat(2.3076375562667102e-36) * t137 * t124 * t84 * t769 * t58 * t331 + f64x8::splat(1.928232771230233e-08) * t137 * t108 * t84 * t147 - f64x8::splat(1.1730960757575554e-16) * t137 * t113 * t84 * t263 + f64x8::splat(8.3129981569617e-11) * t379 * t216 * t24 * t28 * t250 + f64x8::splat(1.8473329237692666e-19) * t789 * t208 * t36 * t262 - f64x8::splat(2.8419748659562924e-11) * t215 * t425 * t20 * t146 - f64x8::splat(1.8946499106375283e-19) * t390 * t250 * t36 * t262 + f64x8::splat(1.2788886896803316e-10) * t374 * t385 + f64x8::splat(1.2630999404250188e-18) * t806 * t392 - f64x8::splat(3.631412328721929e-10) * t811 * t217 - f64x8::splat(1.246949723544255e-10) * t816 * t381 - f64x8::splat(5.4727958326504795e-11) * t821 * t444 * t20 * t146;
            let t828 = ((t2).select(f64x8::splat(0.0), t762 + t826));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t828 + f64x8::splat(8.0) * t462;
            acc_v4rho4 = tv4rho40;
            let t844 = t750 * t88;
            let t909 = t137 * t372;
            let t912 = -f64x8::splat(3.504724538580066e-27) * t365 * t477 + f64x8::splat(0.0091313622465) * t274 * t275 * t173 * t425 - f64x8::splat(0.006087574831) * t6 * t17 * t191 * t277 - f64x8::splat(0.01803467785225591) * t536 * t541 + f64x8::splat(0.05365363551208549) * t274 * t844 * t173 * t444 - f64x8::splat(3.595679962744902e-09) * t454 * t183 + f64x8::splat(0.018262724493) * t536 * t467 - f64x8::splat(0.054104033556767725) * t274 * t539 * t300 * t208 + f64x8::splat(0.0091313622465) * t536 * t471 + f64x8::splat(0.0273940867395) * t274 * t275 * t523 * t132 + f64x8::splat(0.0273940867395) * t274 * t275 * t300 * t250 + f64x8::splat(0.0062435) * t18 * t434 * t301 - f64x8::splat(0.003468611111111111) * t18 * t738 * t174 - f64x8::splat(0.00936525) * t18 * t196 * t524 - f64x8::splat(0.00936525) * t18 * t99 * t88 * (-f64x8::splat(9.394264622862098) * t25 * t27 * t399 - f64x8::splat(51.2437037037037) * t39 * t157 * t404 - f64x8::splat(0.8629969657622203) * t40 * t408 - f64x8::splat(5.527527434842249) * t57 * t163 * t412 - f64x8::splat(138.67713513088705) * t69 * t167 * t417 - f64x8::splat(0.0005220381312566242) * t70 * t422) + f64x8::splat(1.4587831771083095e-29) * t476 * t698 * t40 * t132 - f64x8::splat(1.2966961574296085e-29) * t365 * t698 * t173 * t49 + f64x8::splat(2.664351436834024e-12) * t308 * t144 * t182 * t425 + f64x8::splat(6.216820019279389e-11) * t909 * t310;
            let t927 = t137 * t377;
            let t930 = t179 * t442;
            let t931 = t137 * t930;
            let t946 = t480 * t38;
            let t962 = t137 * t313 * t206 * t259;
            let t979 = t300 * t36;
            let t989 = f64x8::splat(3.134695352136581e-17) * t458 * t318 - f64x8::splat(1.865046005783817e-11) * t546 * t552 - f64x8::splat(0.054104033556767725) * t274 * t539 * t470 * t132 + f64x8::splat(8.653640836000164e-37) * t137 * t74 * t84 * t769 * t49 * t331 + f64x8::splat(1.818468346835372e-11) * t927 * t487 + f64x8::splat(5.130746093109824e-12) * t931 * t144 * t182 * t444 - f64x8::splat(6.234748617721275e-11) * t816 * t529 + f64x8::splat(4.15649907848085e-11) * t379 * t466 * t20 * t146 + f64x8::splat(2.078249539240425e-11) * t379 * t470 * t20 * t146 + f64x8::splat(9.236664618846333e-20) * t789 * t946 * t41 * t132 - f64x8::splat(4.104596874487859e-11) * t821 * t540 * t20 * t146 - f64x8::splat(7.793435772151594e-12) * t577 * t25 * t27 * t132 * t250 - f64x8::splat(3.463749232067375e-20) * t962 * t39 * t26 * t208 * v_sigma + f64x8::splat(3.5524685824453655e-20) * t492 * t39 * t26 * t250 * v_sigma + f64x8::splat(6.394443448401658e-11) * t374 * t501 - f64x8::splat(2.1314811494672192e-11) * t215 * t523 * t20 * t146 - f64x8::splat(9.473249553187641e-20) * t390 * t979 * t262 - f64x8::splat(9.078530821804823e-11) * t811 * t281 + f64x8::splat(3.157749851062547e-19) * t806 * t481 - f64x8::splat(2.960390485371138e-19) * t390 * t495;
            let t991 = ((t2).select(f64x8::splat(0.0), t912 + t989));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t991 + f64x8::splat(6.0) * t556;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1061 = t300 * t300;
            let t1075 = f64x8::splat(9.978729589012688e-28) * t476 * t628 - f64x8::splat(0.01803467785225591) * t274 * t539 * t350 * t208 - f64x8::splat(5.334250494181997e-18) * t257 * t358 + f64x8::splat(0.05365363551208549) * t274 * t844 * t325 * t208 - f64x8::splat(0.012023118568170605) * t536 * t563 + f64x8::splat(0.012175149662) * t536 * t571 - f64x8::splat(0.01803467785225591) * t274 * t539 * t325 * t250 + f64x8::splat(0.018262724493) * t274 * t275 * t173 * t523 + f64x8::splat(0.006087574831) * t536 * t592 + f64x8::splat(0.018262724493) * t274 * t275 * t612 * t132 + f64x8::splat(0.0091313622465) * t274 * t275 * t350 * t250 - f64x8::splat(3.646957942770774e-30) * t627 * t698 * t132 * v_sigma + f64x8::splat(9.725221180722063e-30) * t476 * t698 * t173 * t40 - f64x8::splat(0.0062435) * t18 * t196 * t613 - f64x8::splat(0.00936525) * t18 * t99 * t88 * (f64x8::splat(6.987777777777778) * t39 * t26 * t227 + f64x8::splat(0.17259939315244405) * v_sigma * t231 + f64x8::splat(1.3091512345679013) * t57 * t340 * t236 + f64x8::splat(36.17664394718793) * t69 * t344 * t242 + f64x8::splat(0.00014501059201572894) * t58 * t247) - f64x8::splat(0.0020291916103333334) * t18 * t742 * t326 + f64x8::splat(0.018262724493) * t18 * t207 * t88 * t1061 + f64x8::splat(0.0020811666666666665) * t18 * t434 * t351 - f64x8::splat(2.4867280077117558e-11) * t546 * t585 + f64x8::splat(5.328702873668048e-12) * t308 * t143 * t523 * t331;
            let t1076 = t137 * t490;
            let t1095 = t355 * t206;
            let t1096 = t137 * t1095;
            let t1121 = t137 * t930 * t143;
            let t1131 = t350 * t36;
            let t1150 = t325 * t36;
            let t1154 = f64x8::splat(1.0261492186219648e-11) * t1121 * t280 * t486 + f64x8::splat(2.1314811494672192e-11) * t374 * t596 - f64x8::splat(1.4209874329781462e-11) * t215 * t612 * t20 * t146 - f64x8::splat(3.1577498510625473e-20) * t390 * t1131 * t262 + f64x8::splat(2.4246244624471625e-11) * t379 * t578 - f64x8::splat(1.973593656914092e-19) * t390 * t581 - f64x8::splat(1.0391247696202125e-11) * t577 * t500 * t309 - f64x8::splat(5.1956238481010625e-12) * t577 * t280 * t551 + f64x8::splat(4.736624776593821e-20) * t492 * t979 * t317 - f64x8::splat(2.078249539240425e-11) * t816 * t567 + f64x8::splat(3.078888206282111e-20) * t789 * t1150 * t262;
            let t1157 = ((t2).select(f64x8::splat(0.0), t1075 + f64x8::splat(4.4405857280567066e-20) * t1076 * t622 - f64x8::splat(4.440585728056707e-21) * t620 * t260 * t316 * t250 - f64x8::splat(3.245115313500061e-37) * t137 / t19 / t234 * t84 * t769 * t40 * t331 - f64x8::splat(0.07213871140902364) * t274 * t539 * t276 * t300 + f64x8::splat(4.329686540084219e-21) * t1096 * t260 * t316 * t208 + f64x8::splat(4.144546679519593e-11) * t909 * t332 - f64x8::splat(4.6183323094231665e-20) * t962 * t946 * t494 + f64x8::splat(1.38549969282695e-11) * t379 * t591 * t20 * t146 - f64x8::splat(2.7363979163252397e-11) * t821 * t562 * t20 * t146 + f64x8::splat(2.7709993856539e-11) * t379 * t280 * t24 * t28 * t300 + t1154));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1157 + f64x8::splat(4.0) * t632;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1160 = t137 * t672;
            let t1222 = f64x8::splat(6.8380461426952005e-31) * t1160 * t698 * t132 - f64x8::splat(2.0991839684203523e-28) * t137 * t626 * t84 * t367 + f64x8::splat(0.0091313622465) * t274 * t275 * t666 * t132 + f64x8::splat(0.0091313622465) * t536 * t645 + f64x8::splat(0.05365363551208549) * t274 * t844 * t635 * t132 - f64x8::splat(0.054104033556767725) * t274 * t539 * t325 * t300 + f64x8::splat(0.0273940867395) * t274 * t275 * t300 * t350 + f64x8::splat(0.0273940867395) * t274 * t275 * t173 * t612 - f64x8::splat(0.00936525) * t18 * t99 * t88 * (-f64x8::splat(0.019177710350271562) * t113 - f64x8::splat(0.22442592592592592) * t57 * t28 * t118 - f64x8::splat(7.5718557098765435) * t69 * t41 * t124 - f64x8::splat(3.412013929781857e-05) * t49 * t129) - f64x8::splat(0.006011559284085303) * t18 * t746 * t636 - f64x8::splat(0.00312175) * t18 * t196 * t667 - f64x8::splat(5.4704369141561604e-30) * t627 * t698 * t173 * v_sigma - f64x8::splat(0.054104033556767725) * t274 * t539 * t644 * t132 - f64x8::splat(1.332175718417012e-20) * t620 * t259 * t300 * t649;
            let t1237 = t769 * t20;
            let t1274 = -f64x8::splat(1.865046005783817e-11) * t546 * t654 + f64x8::splat(1.818468346835372e-11) * t927 * t641 + f64x8::splat(6.66087859208506e-20) * t1076 * t650 + f64x8::splat(7.993054310502073e-12) * t308 * t143 * t612 * t331 + f64x8::splat(1.216918242562523e-37) * t137 / t19 / t116 * t84 * t1237 * t146 + f64x8::splat(2.078249539240425e-11) * t379 * t644 * t20 * t146 + f64x8::splat(1.2989059620252656e-20) * t137 * t1095 * t259 * t480 * t621 - f64x8::splat(7.793435772151594e-12) * t577 * t595 * t309 + f64x8::splat(3.5524685824453655e-20) * t492 * t1131 * t317 - f64x8::splat(7.104937164890731e-12) * t215 * t666 * t20 * t146 - f64x8::splat(1.3681989581626199e-11) * t821 * t635 * t20 * t146 + f64x8::splat(1.5392238279329475e-11) * t1121 * t566 * t309 - f64x8::splat(1.5586871544303188e-11) * t577 * t280 * t182 * t300 - f64x8::splat(3.463749232067375e-20) * t962 * t1150 * t317;
            let t1276 = ((t2).select(f64x8::splat(0.0), t1222 + t1274));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t1276 + f64x8::splat(2.0) * t678;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1279 = t325 * t325;
            let t1300 = t350 * t350;
            let t1340 = f64x8::splat(0.05365363551208549) * t18 * t751 * t88 * t1279 + f64x8::splat(2.0522984372439296e-11) * t931 * t143 * t635 * t331 - f64x8::splat(0.10820806711353545) * t274 * t539 * t325 * t350 + f64x8::splat(2.597811924050531e-20) * t1096 * t259 * t325 * t649 - f64x8::splat(3.1173743088606375e-11) * t577 * t280 * t182 * t350 + f64x8::splat(0.0273940867395) * t18 * t207 * t88 * t1300 + f64x8::splat(0.036525448986) * t274 * t275 * t173 * t666 + f64x8::splat(2.7352184570780802e-30) * t1160 * t698 * t173 - f64x8::splat(2.664351436834024e-20) * t620 * t259 * t350 * t649 + f64x8::splat(1.0657405747336096e-11) * t308 * t143 * t666 * t331 - f64x8::splat(0.00936525) * t18 * t99 * t88 * (f64x8::splat(0.021039930555555555) * t57 * t27 * t62 + f64x8::splat(1.1357783564814814) * t69 * t157 * t74 + f64x8::splat(6.397526118340983e-06) * t40 * t80) - f64x8::splat(4.563443409609461e-38) * t137 / t19 / t60 * t84 * t1237 * t182;
            let t1341 = ((t2).select(f64x8::splat(0.0), t1340));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t1341;
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
