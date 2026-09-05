//! GGA_K_VT84F fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_vt84f.c`
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
pub fn gga_k_vt84f_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_mu = f64x8::splat(param_mu);
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = t24 * t24;
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t29 = t25 / t27;
            let t30 = ((v_sigma).sqrt());
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t30 * t31;
            let t34 = f64x8::splat(1.0) / t21 / v_rho;
            let t37 = t29 * t32 * t34 / f64x8::splat(12.0);
            let t38 = ((f64x8::splat(f64::EPSILON)).sqrt());
            let t39 = (t37).simd_le(t38);
            let t41 = (-param_mu + param_alpha + f64x8::splat(5.0) / f64x8::splat(3.0)) * t24;
            let t42 = t27 * t27;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = t31 * t31;
            let t46 = v_sigma * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t53 = param_mu * param_alpha;
            let t54 = param_mu * param_mu;
            let t56 = (t53 + t54 - param_alpha) * t25;
            let t58 = f64x8::splat(1.0) / t27 / t26;
            let t59 = t56 * t58;
            let t60 = v_sigma * v_sigma;
            let t61 = t60 * t31;
            let t62 = t47 * t47;
            let t63 = t62 * v_rho;
            let t65 = f64x8::splat(1.0) / t21 / t63;
            let t69 = param_alpha * param_alpha;
            let t71 = param_mu * t69 / f64x8::splat(2.0);
            let t74 = t69 / f64x8::splat(2.0);
            let t76 = t26 * t26;
            let t78 = (-t71 - (t53 + t54) * param_mu - t74) / t76;
            let t79 = t60 * v_sigma;
            let t80 = t62 * t62;
            let t81 = f64x8::splat(1.0) / t80;
            let t85 = t69 * param_alpha;
            let t89 = t54 * param_mu;
            let t93 = (param_mu * t85 / f64x8::splat(6.0) - (-param_alpha * t54 - t71 - t89) * param_mu + t74) * t24;
            let t95 = f64x8::splat(1.0) / t42 / t76;
            let t96 = t93 * t95;
            let t97 = t60 * t60;
            let t98 = t97 * t45;
            let t99 = t80 * t47;
            let t101 = f64x8::splat(1.0) / t22 / t99;
            let t106 = (t38).simd_lt(t37);
            let t107 = ((t106).select(t37, t38));
            let t108 = t107 * t107;
            let t109 = param_mu * t108;
            let t110 = param_alpha * t108;
            let t111 = (simd::exp(-t110));
            let t112 = f64x8::splat(1.0) + t109;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t111 * t113;
            let t116 = t108 * t108;
            let t118 = (simd::exp(-param_alpha * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t108;
            let t121 = t120 - f64x8::splat(1.0);
            let t125 = ((t39).select(f64x8::splat(1.0) + t44 * t46 * t49 / f64x8::splat(24.0) + t59 * t61 * t65 / f64x8::splat(288.0) + t78 * t79 * t81 / f64x8::splat(576.0) + t96 * t98 * t101 / f64x8::splat(13824.0), f64x8::splat(1.0) - t109 * t114 + t119 * t121 + f64x8::splat(5.0) / f64x8::splat(3.0) * t108));
            let t129 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t125));
            let tzk0 = f64x8::splat(2.0) * t129;
            acc_zk = tzk0;
            let t131 = t20 / t21;
            let t135 = t47 * v_rho;
            let t137 = f64x8::splat(1.0) / t22 / t135;
            let t141 = t62 * t47;
            let t143 = f64x8::splat(1.0) / t21 / t141;
            let t147 = t80 * v_rho;
            let t148 = f64x8::splat(1.0) / t147;
            let t152 = t80 * t135;
            let t154 = f64x8::splat(1.0) / t22 / t152;
            let t159 = param_mu * t107;
            let t161 = f64x8::splat(1.0) / t21 / t47;
            let t165 = ((t106).select(-t29 * t32 * t161 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t166 = t114 * t165;
            let t169 = t108 * t107;
            let t170 = param_mu * t169;
            let t171 = t170 * param_alpha;
            let t174 = t54 * t169;
            let t175 = t112 * t112;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t111 * t176;
            let t178 = t177 * t165;
            let t181 = param_alpha * t169;
            let t182 = t165 * t118;
            let t183 = t182 * t121;
            let t187 = t119 / t169;
            let t190 = t107 * t165;
            let t193 = ((t39).select(-t44 * t46 * t137 / f64x8::splat(9.0) - t59 * t61 * t143 / f64x8::splat(54.0) - t78 * t79 * t148 / f64x8::splat(72.0) - t96 * t98 * t154 / f64x8::splat(1296.0), -f64x8::splat(2.0) * t159 * t166 + f64x8::splat(2.0) * t171 * t166 + f64x8::splat(2.0) * t174 * t178 + f64x8::splat(4.0) * t181 * t183 - f64x8::splat(2.0) * t187 * t165 + f64x8::splat(10.0) / f64x8::splat(3.0) * t190));
            let t198 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t125 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t193));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t198 + f64x8::splat(2.0) * t129;
            acc_vrho = tvrho0;
            let t201 = t43 * t45;
            let t205 = v_sigma * t31;
            let t212 = t79 * t45;
            let t218 = f64x8::splat(1.0) / t30 * t31;
            let t222 = ((t106).select(t29 * t218 * t34 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t223 = t114 * t222;
            let t228 = t177 * t222;
            let t231 = t222 * t118;
            let t232 = t231 * t121;
            let t240 = ((t39).select(t41 * t201 * t49 / f64x8::splat(24.0) + t59 * t205 * t65 / f64x8::splat(144.0) + t78 * t60 * t81 / f64x8::splat(192.0) + t96 * t212 * t101 / f64x8::splat(3456.0), -f64x8::splat(2.0) * t159 * t223 + f64x8::splat(2.0) * t171 * t223 + f64x8::splat(2.0) * t174 * t228 + f64x8::splat(4.0) * t181 * t232 - f64x8::splat(2.0) * t187 * t222 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t222));
            let t244 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t240));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t244;
            acc_vsigma = tvsigma0;
            let t247 = t20 * t34;
            let t255 = f64x8::splat(1.0) / t22 / t62;
            let t261 = f64x8::splat(1.0) / t21 / t62 / t135;
            let t265 = f64x8::splat(1.0) / t99;
            let t269 = t80 * t62;
            let t271 = f64x8::splat(1.0) / t22 / t269;
            let t276 = t165 * t165;
            let t277 = param_mu * t276;
            let t280 = t109 * param_alpha;
            let t281 = t276 * t111;
            let t282 = t281 * t113;
            let t285 = t54 * t108;
            let t286 = t177 * t276;
            let t290 = f64x8::splat(1.0) / t21 / t135;
            let t294 = ((t106).select(f64x8::splat(7.0) / f64x8::splat(27.0) * t29 * t32 * t290, f64x8::splat(0.0)));
            let t295 = t114 * t294;
            let t300 = param_mu * t116;
            let t301 = t300 * t69;
            let t304 = t54 * t116;
            let t305 = t304 * param_alpha;
            let t308 = t89 * t116;
            let t310 = f64x8::splat(1.0) / t175 / t112;
            let t311 = t111 * t310;
            let t319 = t276 * t118 * t121;
            let t322 = t294 * t118;
            let t323 = t322 * t121;
            let t326 = t116 * t108;
            let t327 = t69 * t326;
            let t330 = param_alpha * t276;
            let t334 = t119 / t116;
            let t340 = t107 * t294;
            let t342 = -f64x8::splat(2.0) * t277 * t114 + f64x8::splat(10.0) * t280 * t282 + f64x8::splat(10.0) * t285 * t286 - f64x8::splat(2.0) * t159 * t295 + f64x8::splat(2.0) * t171 * t295 - f64x8::splat(4.0) * t301 * t282 - f64x8::splat(8.0) * t305 * t286 - f64x8::splat(8.0) * t308 * t311 * t276 + f64x8::splat(2.0) * t174 * t177 * t294 + f64x8::splat(12.0) * t110 * t319 + f64x8::splat(4.0) * t181 * t323 - f64x8::splat(16.0) * t327 * t319 - f64x8::splat(16.0) * t330 * t118 + f64x8::splat(6.0) * t334 * t276 - f64x8::splat(2.0) * t187 * t294 + f64x8::splat(10.0) / f64x8::splat(3.0) * t276 + f64x8::splat(10.0) / f64x8::splat(3.0) * t340;
            let t343 = ((t39).select(f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t46 * t255 + f64x8::splat(19.0) / f64x8::splat(162.0) * t59 * t61 * t261 + t78 * t79 * t265 / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(3888.0) * t96 * t98 * t271, t342));
            let t348 = ((t2).select(f64x8::splat(0.0), -t7 * t247 * t125 / f64x8::splat(30.0) + t7 * t131 * t193 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t343));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t348 + f64x8::splat(4.0) * t198;
            acc_v2rho2 = tv2rho20;
            let t367 = param_mu * t165;
            let t370 = t165 * t111;
            let t371 = t113 * t222;
            let t372 = t370 * t371;
            let t375 = t285 * t111;
            let t376 = t176 * t222;
            let t383 = ((t106).select(-t29 * t218 * t161 / f64x8::splat(18.0), f64x8::splat(0.0)));
            let t384 = t114 * t383;
            let t391 = t222 * t111;
            let t392 = t176 * t165;
            let t393 = t391 * t392;
            let t396 = t308 * t111;
            let t397 = t310 * t222;
            let t404 = t110 * t222;
            let t407 = t383 * t118;
            let t408 = t407 * t121;
            let t411 = t327 * t222;
            let t414 = param_alpha * t222;
            let t417 = t165 * t222;
            let t425 = -f64x8::splat(2.0) * t367 * t223 + f64x8::splat(10.0) * t280 * t372 + f64x8::splat(10.0) * t375 * t376 * t165 - f64x8::splat(2.0) * t159 * t384 + f64x8::splat(2.0) * t171 * t384 - f64x8::splat(4.0) * t301 * t372 - f64x8::splat(8.0) * t305 * t393 - f64x8::splat(8.0) * t396 * t397 * t165 + f64x8::splat(2.0) * t174 * t177 * t383 + f64x8::splat(12.0) * t404 * t183 + f64x8::splat(4.0) * t181 * t408 - f64x8::splat(16.0) * t411 * t183 - f64x8::splat(16.0) * t414 * t182 + f64x8::splat(6.0) * t334 * t417 - f64x8::splat(2.0) * t187 * t383 + f64x8::splat(10.0) / f64x8::splat(3.0) * t417 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t383;
            let t426 = ((t39).select(-t41 * t201 * t137 / f64x8::splat(9.0) - t59 * t205 * t143 / f64x8::splat(27.0) - t78 * t60 * t148 / f64x8::splat(24.0) - t96 * t212 * t154 / f64x8::splat(324.0), t425));
            let t431 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t240 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t426));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t431 + f64x8::splat(2.0) * t244;
            acc_v2rhosigma = tv2rhosigma0;
            let t434 = t58 * t31;
            let t441 = t60 * t45;
            let t446 = t222 * t222;
            let t447 = param_mu * t446;
            let t450 = t446 * t111;
            let t451 = t450 * t113;
            let t454 = t177 * t446;
            let t459 = f64x8::splat(1.0) / t30 / v_sigma * t31;
            let t463 = ((t106).select(-t29 * t459 * t34 / f64x8::splat(48.0), f64x8::splat(0.0)));
            let t464 = t114 * t463;
            let t479 = t446 * t118;
            let t480 = t479 * t121;
            let t484 = t463 * t118 * t121;
            let t499 = -f64x8::splat(2.0) * t447 * t114 + f64x8::splat(10.0) * t280 * t451 + f64x8::splat(10.0) * t285 * t454 - f64x8::splat(2.0) * t159 * t464 + f64x8::splat(2.0) * t171 * t464 - f64x8::splat(4.0) * t301 * t451 - f64x8::splat(8.0) * t305 * t454 - f64x8::splat(8.0) * t308 * t311 * t446 + f64x8::splat(2.0) * t174 * t177 * t463 + f64x8::splat(12.0) * t110 * t480 + f64x8::splat(4.0) * t181 * t484 - f64x8::splat(16.0) * t327 * t480 - f64x8::splat(16.0) * param_alpha * t446 * t118 + f64x8::splat(6.0) * t334 * t446 - f64x8::splat(2.0) * t187 * t463 + f64x8::splat(10.0) / f64x8::splat(3.0) * t446 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t463;
            let t500 = ((t39).select(t56 * t434 * t65 / f64x8::splat(144.0) + t78 * v_sigma * t81 / f64x8::splat(96.0) + t96 * t441 * t101 / f64x8::splat(1152.0), t499));
            let t504 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t500));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t504;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
