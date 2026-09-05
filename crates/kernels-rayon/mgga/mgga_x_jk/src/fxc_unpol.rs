//! MGGA_X_JK fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`
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
pub fn mgga_x_jk_fxc_unpol(
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
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t6 = f64x8::splat(1.0) / t5;
            let t7 = t4 * t6;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = t4 * t4;
            let t22 = param_beta * t21;
            let t24 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = f64x8::splat(M_CBRT4);
            let t27 = t25 * t26;
            let t28 = t22 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t34 = t33 * t32;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = param_gamma * param_beta;
            let t37 = ((v_sigma).sqrt());
            let t38 = t36 * t37;
            let t40 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = t29 * t40;
            let t44 = (simd::ln(t37 * t29 * t40 + ((((t37 * t29 * t40) * (t37 * t29 * t40)) + f64x8::splat(1.0)).sqrt())));
            let t45 = t41 * t44;
            let t47 = t38 * t45 + f64x8::splat(1.0);
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t35 * t48;
            let t50 = t31 * t35;
            let t51 = v_lapl * t30;
            let t52 = t33 * v_rho;
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = -t51 * t53 + t50;
            let t56 = f64x8::splat(1.0) / v_sigma;
            let t57 = t55 * t56;
            let t58 = t29 * t34;
            let t60 = t57 * t58 + f64x8::splat(1.0);
            let t61 = f64x8::splat(1.0) / t60;
            let t66 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t31 * t49 * t61;
            let t70 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
            let t72 = t18 / t33;
            let t76 = t32 * v_rho;
            let t78 = f64x8::splat(1.0) / t33 / t76;
            let t79 = t78 * t48;
            let t85 = t22 * t27 * v_sigma;
            let t86 = t30 * t35;
            let t87 = t47 * t47;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t88 * t61;
            let t91 = f64x8::splat(1.0) / t19 / t32;
            let t93 = t29 * t91 * t44;
            let t95 = t36 * v_sigma;
            let t96 = t30 * t78;
            let t97 = t50 + f64x8::splat(1.0);
            let t98 = ((t97).sqrt());
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t96 * t99;
            let t103 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t95 * t100 - f64x8::splat(4.0) / f64x8::splat(3.0) * t38 * t93;
            let t104 = t89 * t103;
            let t105 = t86 * t104;
            let t108 = t60 * t60;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t48 * t109;
            let t115 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t31 * t78 + f64x8::splat(5.0) / f64x8::splat(3.0) * t51 * t35;
            let t116 = t115 * t56;
            let t118 = t29 * t52;
            let t121 = t116 * t58 + f64x8::splat(8.0) / f64x8::splat(3.0) * t57 * t118;
            let t122 = t110 * t121;
            let t123 = t86 * t122;
            let t126 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t28 * t31 * t79 * t61 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t105 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t123;
            let t131 = ((t3).select(f64x8::splat(0.0), -t7 * t72 * t66 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t126));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t131 + f64x8::splat(2.0) * t70;
            acc_vrho = tvrho0;
            let t134 = t48 * t61;
            let t138 = t36 / t37;
            let t140 = t86 * t99;
            let t143 = t138 * t45 / f64x8::splat(2.0) + t36 * t140 / f64x8::splat(2.0);
            let t144 = t89 * t143;
            let t145 = t86 * t144;
            let t148 = v_sigma * v_sigma;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t55 * t149;
            let t152 = -t150 * t58 + f64x8::splat(2.0) * t56;
            let t153 = t110 * t152;
            let t154 = t86 * t153;
            let t157 = f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t86 * t134 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t145 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t154;
            let t161 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t157));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t161;
            acc_vsigma = tvsigma0;
            let t163 = t6 * t18;
            let t164 = t40 * param_beta;
            let t166 = t30 * t48;
            let t168 = t27 * t166 * t109;
            let t171 = ((t3).select(f64x8::splat(0.0), -t163 * t164 * t168 / f64x8::splat(2.0)));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t171;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t174 = t18 * t53;
            let t181 = t32 * t32;
            let t183 = f64x8::splat(1.0) / t33 / t181;
            let t189 = t96 * t104;
            let t192 = t96 * t122;
            let t196 = f64x8::splat(1.0) / t87 / t47;
            let t197 = t196 * t61;
            let t198 = t103 * t103;
            let t199 = t197 * t198;
            let t200 = t86 * t199;
            let t203 = t86 * t88;
            let t204 = t109 * t103;
            let t205 = t204 * t121;
            let t210 = f64x8::splat(1.0) / t19 / t76;
            let t212 = t29 * t210 * t44;
            let t215 = t30 * t183;
            let t216 = t215 * t99;
            let t219 = t36 * t148;
            let t222 = f64x8::splat(1.0) / t19 / t181 / t76;
            let t225 = f64x8::splat(1.0) / t98 / t97;
            let t226 = t29 * t222 * t225;
            let t229 = f64x8::splat(28.0) / f64x8::splat(9.0) * t38 * t212 + f64x8::splat(20.0) / f64x8::splat(3.0) * t95 * t216 - f64x8::splat(32.0) / f64x8::splat(9.0) * t219 * t226;
            let t230 = t89 * t229;
            let t231 = t86 * t230;
            let t235 = f64x8::splat(1.0) / t108 / t60;
            let t236 = t48 * t235;
            let t237 = t121 * t121;
            let t238 = t236 * t237;
            let t239 = t86 * t238;
            let t246 = f64x8::splat(88.0) / f64x8::splat(9.0) * t31 * t183 - f64x8::splat(40.0) / f64x8::splat(9.0) * t51 * t78;
            let t247 = t246 * t56;
            let t251 = t29 * t33;
            let t254 = t247 * t58 + f64x8::splat(16.0) / f64x8::splat(3.0) * t116 * t118 + f64x8::splat(40.0) / f64x8::splat(9.0) * t57 * t251;
            let t255 = t110 * t254;
            let t256 = t86 * t255;
            let t259 = f64x8::splat(176.0) / f64x8::splat(81.0) * t28 * t31 * t183 * t48 * t61 + f64x8::splat(32.0) / f64x8::splat(27.0) * t85 * t189 + f64x8::splat(32.0) / f64x8::splat(27.0) * t85 * t192 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t200 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t203 * t205 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t231 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t239 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t256;
            let t264 = ((t3).select(f64x8::splat(0.0), t7 * t174 * t66 / f64x8::splat(12.0) - t7 * t72 * t126 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t259));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t264 + f64x8::splat(4.0) * t131;
            acc_v2rho2 = tv2rho20;
            let t277 = t96 * t144;
            let t280 = t86 * t196;
            let t281 = t61 * t143;
            let t282 = t281 * t103;
            let t286 = t109 * t143;
            let t287 = t286 * t121;
            let t295 = t36 * t29;
            let t296 = t181 * t32;
            let t298 = f64x8::splat(1.0) / t19 / t296;
            let t303 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t138 * t93 - f64x8::splat(2.0) * t36 * t100 + f64x8::splat(4.0) / f64x8::splat(3.0) * t295 * t298 * t225 * v_sigma;
            let t304 = t89 * t303;
            let t305 = t86 * t304;
            let t308 = t96 * t153;
            let t311 = t109 * t152;
            let t312 = t311 * t103;
            let t316 = t86 * t48;
            let t317 = t235 * t152;
            let t318 = t317 * t121;
            let t322 = t115 * t149;
            let t326 = -t322 * t58 - f64x8::splat(8.0) / f64x8::splat(3.0) * t150 * t118;
            let t327 = t110 * t326;
            let t328 = t86 * t327;
            let t331 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t28 * t96 * t134 - f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t105 - f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t123 + f64x8::splat(16.0) / f64x8::splat(27.0) * t85 * t277 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t280 * t282 + f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t203 * t287 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t305 + f64x8::splat(16.0) / f64x8::splat(27.0) * t85 * t308 + f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t203 * t312 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t316 * t318 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t328;
            let t336 = ((t3).select(f64x8::splat(0.0), -t7 * t72 * t157 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t331));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t336 + f64x8::splat(2.0) * t161;
            acc_v2rhosigma = tv2rhosigma0;
            let t339 = t91 * param_beta;
            let t344 = t163 * t164 * t25;
            let t345 = t26 * t30;
            let t346 = t88 * t109;
            let t348 = t345 * t346 * t103;
            let t352 = t345 * t236 * t121;
            let t355 = ((t3).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t163 * t339 * t168 + t344 * t348 / f64x8::splat(2.0) + t344 * t352));
            let tv2rholapl0 = f64x8::splat(2.0) * v_rho * t355 + f64x8::splat(2.0) * t171;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let t362 = t143 * t143;
            let t363 = t197 * t362;
            let t364 = t86 * t363;
            let t367 = t286 * t152;
            let t373 = t36 / t37 / v_sigma;
            let t376 = t36 * t56;
            let t379 = t181 * v_rho;
            let t382 = t29 / t19 / t379;
            let t383 = t382 * t225;
            let t386 = -t373 * t45 / f64x8::splat(4.0) + t376 * t140 / f64x8::splat(4.0) - t36 * t383 / f64x8::splat(2.0);
            let t387 = t89 * t386;
            let t388 = t86 * t387;
            let t391 = t152 * t152;
            let t392 = t236 * t391;
            let t393 = t86 * t392;
            let t397 = t148 * v_sigma;
            let t398 = f64x8::splat(1.0) / t397;
            let t399 = t55 * t398;
            let t402 = f64x8::splat(2.0) * t399 * t58 - f64x8::splat(4.0) * t149;
            let t403 = t110 * t402;
            let t404 = t86 * t403;
            let t407 = -f64x8::splat(4.0) / f64x8::splat(9.0) * t28 * t145 - f64x8::splat(4.0) / f64x8::splat(9.0) * t28 * t154 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t364 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t203 * t367 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t388 + f64x8::splat(4.0) / f64x8::splat(9.0) * t85 * t393 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t404;
            let t411 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t407));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t411;
            acc_v2sigma2 = tv2sigma20;
            let t414 = t345 * t346 * t143;
            let t418 = t345 * t236 * t152;
            let t421 = ((t3).select(f64x8::splat(0.0), t344 * t414 / f64x8::splat(2.0) + t344 * t418));
            let tv2sigmalapl0 = f64x8::splat(2.0) * v_rho * t421;
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let t423 = f64x8::splat(1.0) / t19;
            let t426 = t163 * t423 * param_beta * t25;
            let t428 = t345 * t236 * t56;
            let t431 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t426 * t428));
            let tv2lapl20 = f64x8::splat(2.0) * v_rho * t431;
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
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
