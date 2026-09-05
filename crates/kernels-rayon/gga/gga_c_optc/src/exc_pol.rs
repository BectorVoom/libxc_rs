//! GGA_C_OPTC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_optc.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_optc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
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
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = t5 * t23;
            let t25 = t21 * t24;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.03109) * t62 * t71 + t33 - f64x8::splat(0.019751789702565206) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.019751789702565206) * t60 * t85;
            let t92 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t93 = (simd::cbrt(t92));
            let t94 = t93 * t93;
            let t95 = t19 * t94;
            let t96 = t45 * t45;
            let t97 = t47 * t47;
            let t98 = ((t44).select(t96, t97));
            let t99 = t52 * t52;
            let t100 = ((t51).select(t96, t99));
            let t102 = t98 / f64x8::splat(2.0) + t100 / f64x8::splat(2.0);
            let t103 = t102 * t102;
            let t104 = t103 * t102;
            let t105 = f64x8::splat(1.0) / t93;
            let t106 = t19 * t105;
            let t108 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t110 = f64x8::splat(1.0) / t8 / t37;
            let t111 = t108 * t110;
            let t112 = t111 * t56;
            let t113 = f64x8::splat(1.0) / t103;
            let t115 = f64x8::splat(1.0) / t3;
            let t116 = t115 * t5;
            let t117 = t113 * t19 * t116;
            let t120 = -t33 + t89 + t91;
            let t121 = f64x8::splat(1.0) / t104;
            let t123 = f64x8::splat(1.0) / t94;
            let t124 = t1 * t123;
            let t127 = (simd::exp(-f64x8::splat(128.97460341341235) * t120 * t121 * t124));
            let t128 = t127 - f64x8::splat(1.0);
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t105 * t129;
            let t131 = t108 * t108;
            let t133 = f64x8::splat(1.0) / t22 / t38;
            let t134 = t131 * t133;
            let t135 = t130 * t134;
            let t136 = t56 * t56;
            let t137 = t103 * t103;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t136 * t138;
            let t140 = f64x8::splat(1.0) / t20;
            let t141 = t140 * t6;
            let t142 = t139 * t141;
            let t145 = t112 * t117 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t135 * t142;
            let t146 = t1 * t105;
            let t147 = t129 * t108;
            let t148 = t146 * t147;
            let t149 = t110 * t56;
            let t150 = t113 * t115;
            let t151 = t150 * t5;
            let t155 = t19 * t123;
            let t156 = t128 * t128;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t157 * t131;
            let t159 = t155 * t158;
            let t160 = t133 * t136;
            let t161 = t138 * t140;
            let t162 = t161 * t6;
            let t163 = t160 * t162;
            let t166 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t148 * t149 * t151 + f64x8::splat(0.0075571056687546295) * t159 * t163;
            let t167 = f64x8::splat(1.0) / t166;
            let t171 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t106 * t145 * t167;
            let t172 = (simd::ln(t171));
            let t176 = t2 * t93;
            let t179 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t11 + f64x8::splat(0.00184725) * t25;
            let t182 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t11 + f64x8::splat(118.0) * t25;
            let t183 = f64x8::splat(1.0) / t182;
            let t185 = t179 * t183 - f64x8::splat(0.0018535714285714286);
            let t186 = t185 * t102;
            let t187 = t186 * t108;
            let t188 = t176 * t187;
            let t190 = (simd::cbrt(f64x8::splat(9.0)));
            let t191 = t190 * t190;
            let t193 = t2 * t5 * t191 * t3;
            let t195 = f64x8::splat(1.0) / t22 / t37;
            let t197 = t108 * t56;
            let t201 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t193 * t195 * t103 * t197));
            let t202 = t116 * t201;
            let t203 = t149 * t202;
            let t207 = param_c1 * (-t33 + t89 + t91 + f64x8::splat(0.002584488143490343) * t95 * t104 * t172 + t188 * t203 / f64x8::splat(2.0));
            let t208 = param_c2 - param_c1;
            let t209 = t4 * t6;
            let t210 = t9 * t56;
            let t211 = f64x8::splat(1.0) / t43;
            let t212 = (simd::cbrt(t211));
            let t214 = t209 * t210 * t212;
            let t216 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t214;
            let t217 = ((t214).sqrt());
            let t220 = ((t214) * (t214).sqrt());
            let t222 = t21 * t5;
            let t223 = t23 * t136;
            let t224 = t212 * t212;
            let t226 = t222 * t223 * t224;
            let t228 = f64x8::splat(3.79785) * t217 + f64x8::splat(0.8969) * t214 + f64x8::splat(0.204775) * t220 + f64x8::splat(0.123235) * t226;
            let t231 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t228;
            let t232 = (simd::ln(t231));
            let t234 = f64x8::splat(0.062182) * t216 * t232;
            let t235 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t237 = ((t235).select(t46, f64x8::splat(2.0) * t56));
            let t238 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t239 = ((t238).select(t46, f64x8::splat(0.0)));
            let t241 = (t237 + t239 - f64x8::splat(2.0)) * t59;
            let t243 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t214;
            let t248 = f64x8::splat(7.05945) * t217 + f64x8::splat(1.549425) * t214 + f64x8::splat(0.420775) * t220 + f64x8::splat(0.1562925) * t226;
            let t251 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t248;
            let t252 = (simd::ln(t251));
            let t256 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t214;
            let t261 = f64x8::splat(5.1785) * t217 + f64x8::splat(0.905775) * t214 + f64x8::splat(0.1100325) * t220 + f64x8::splat(0.1241775) * t226;
            let t264 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t261;
            let t265 = (simd::ln(t264));
            let t266 = t256 * t265;
            let t269 = t241 * (-f64x8::splat(0.03109) * t243 * t252 + t234 - f64x8::splat(0.019751789702565206) * t266);
            let t271 = f64x8::splat(0.019751789702565206) * t241 * t266;
            let t272 = ((t235).select(t96, t136));
            let t273 = ((t238).select(t96, f64x8::splat(0.0)));
            let t275 = t272 / f64x8::splat(2.0) + t273 / f64x8::splat(2.0);
            let t276 = t275 * t275;
            let t277 = t276 * t275;
            let t278 = v_rho0 * v_rho0;
            let t279 = (simd::cbrt(v_rho0));
            let t280 = t279 * t279;
            let t282 = f64x8::splat(1.0) / t280 / t278;
            let t283 = v_sigma0 * t282;
            let t284 = f64x8::splat(1.0) / t276;
            let t285 = t284 * t19;
            let t286 = t283 * t285;
            let t287 = f64x8::splat(1.0) / t212;
            let t288 = t8 * t287;
            let t289 = t116 * t288;
            let t293 = f64x8::splat(1.0) / t277;
            let t297 = (simd::exp(-f64x8::splat(128.97460341341235) * (-t234 + t269 + t271) * t293 * t124));
            let t298 = t297 - f64x8::splat(1.0);
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t105 * t299;
            let t301 = v_sigma0 * v_sigma0;
            let t302 = t278 * t278;
            let t303 = t302 * v_rho0;
            let t305 = f64x8::splat(1.0) / t279 / t303;
            let t306 = t301 * t305;
            let t307 = t300 * t306;
            let t308 = t276 * t276;
            let t309 = f64x8::splat(1.0) / t308;
            let t310 = t309 * t140;
            let t311 = t6 * t22;
            let t312 = f64x8::splat(1.0) / t224;
            let t313 = t311 * t312;
            let t314 = t310 * t313;
            let t317 = t286 * t289 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t307 * t314;
            let t318 = t299 * v_sigma0;
            let t320 = t146 * t318 * t282;
            let t321 = t284 * t115;
            let t322 = t5 * t8;
            let t323 = t322 * t287;
            let t324 = t321 * t323;
            let t327 = t298 * t298;
            let t328 = f64x8::splat(1.0) / t327;
            let t329 = t328 * t301;
            let t330 = t329 * t305;
            let t331 = t155 * t330;
            let t334 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t320 * t324 + f64x8::splat(0.0075571056687546295) * t331 * t314;
            let t335 = f64x8::splat(1.0) / t334;
            let t339 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t106 * t317 * t335;
            let t340 = (simd::ln(t339));
            let t346 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t214 + f64x8::splat(0.00184725) * t226;
            let t349 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t214 + f64x8::splat(118.0) * t226;
            let t350 = f64x8::splat(1.0) / t349;
            let t352 = t346 * t350 - f64x8::splat(0.0018535714285714286);
            let t353 = t352 * t275;
            let t355 = t176 * t353 * v_sigma0;
            let t356 = t282 * t115;
            let t357 = t356 * t5;
            let t358 = t56 * t276;
            let t362 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t193 * t358 * t283));
            let t363 = t288 * t362;
            let t364 = t357 * t363;
            let t367 = -t234 + t269 + t271 + f64x8::splat(0.002584488143490343) * t95 * t277 * t340 + t355 * t364 / f64x8::splat(2.0);
            let t368 = ((t44).select(zeta_threshold, t43));
            let t370 = f64x8::splat(1.0) / t50;
            let t371 = (simd::cbrt(t370));
            let t373 = t209 * t210 * t371;
            let t375 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t373;
            let t376 = ((t373).sqrt());
            let t379 = ((t373) * (t373).sqrt());
            let t381 = t371 * t371;
            let t383 = t222 * t223 * t381;
            let t385 = f64x8::splat(3.79785) * t376 + f64x8::splat(0.8969) * t373 + f64x8::splat(0.204775) * t379 + f64x8::splat(0.123235) * t383;
            let t388 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t385;
            let t389 = (simd::ln(t388));
            let t391 = f64x8::splat(0.062182) * t375 * t389;
            let t393 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t373;
            let t398 = f64x8::splat(7.05945) * t376 + f64x8::splat(1.549425) * t373 + f64x8::splat(0.420775) * t379 + f64x8::splat(0.1562925) * t383;
            let t401 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t398;
            let t402 = (simd::ln(t401));
            let t406 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t373;
            let t411 = f64x8::splat(5.1785) * t376 + f64x8::splat(0.905775) * t373 + f64x8::splat(0.1100325) * t379 + f64x8::splat(0.1241775) * t383;
            let t414 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t411;
            let t415 = (simd::ln(t414));
            let t416 = t406 * t415;
            let t419 = t241 * (-f64x8::splat(0.03109) * t393 * t402 + t391 - f64x8::splat(0.019751789702565206) * t416);
            let t421 = f64x8::splat(0.019751789702565206) * t241 * t416;
            let t422 = v_rho1 * v_rho1;
            let t423 = (simd::cbrt(v_rho1));
            let t424 = t423 * t423;
            let t426 = f64x8::splat(1.0) / t424 / t422;
            let t427 = v_sigma2 * t426;
            let t428 = t427 * t285;
            let t429 = f64x8::splat(1.0) / t371;
            let t430 = t8 * t429;
            let t431 = t116 * t430;
            let t438 = (simd::exp(-f64x8::splat(128.97460341341235) * (-t391 + t419 + t421) * t293 * t124));
            let t439 = t438 - f64x8::splat(1.0);
            let t440 = f64x8::splat(1.0) / t439;
            let t441 = t105 * t440;
            let t442 = v_sigma2 * v_sigma2;
            let t443 = t422 * t422;
            let t444 = t443 * v_rho1;
            let t446 = f64x8::splat(1.0) / t423 / t444;
            let t447 = t442 * t446;
            let t448 = t441 * t447;
            let t449 = f64x8::splat(1.0) / t381;
            let t450 = t311 * t449;
            let t451 = t310 * t450;
            let t454 = t428 * t431 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t448 * t451;
            let t455 = t440 * v_sigma2;
            let t457 = t146 * t455 * t426;
            let t458 = t322 * t429;
            let t459 = t321 * t458;
            let t462 = t439 * t439;
            let t463 = f64x8::splat(1.0) / t462;
            let t464 = t463 * t442;
            let t465 = t464 * t446;
            let t466 = t155 * t465;
            let t469 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t457 * t459 + f64x8::splat(0.0075571056687546295) * t466 * t451;
            let t470 = f64x8::splat(1.0) / t469;
            let t474 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t106 * t454 * t470;
            let t475 = (simd::ln(t474));
            let t481 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t373 + f64x8::splat(0.00184725) * t383;
            let t484 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t373 + f64x8::splat(118.0) * t383;
            let t485 = f64x8::splat(1.0) / t484;
            let t487 = t481 * t485 - f64x8::splat(0.0018535714285714286);
            let t488 = t487 * t275;
            let t490 = t176 * t488 * v_sigma2;
            let t491 = t426 * t115;
            let t492 = t491 * t5;
            let t496 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t193 * t358 * t427));
            let t497 = t430 * t496;
            let t498 = t492 * t497;
            let t501 = -t391 + t419 + t421 + f64x8::splat(0.002584488143490343) * t95 * t277 * t475 + t490 * t498 / f64x8::splat(2.0);
            let t502 = ((t51).select(zeta_threshold, t50));
            let t506 = t208 * (t367 * t368 / f64x8::splat(2.0) + t501 * t502 / f64x8::splat(2.0));
            let tzk0 = t207 + t506;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
