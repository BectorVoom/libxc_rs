//! MGGA_X_GVT4 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`
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
pub fn mgga_x_gvt4_fxc_unpol(
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
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t7 = zeta_threshold - f64x8::splat(1.0);
            let t9 = ((t6).select(t7, (t6).select(-t7, f64x8::splat(0.0))));
            let t10 = f64x8::splat(1.0) + t9;
            let t12 = (simd::cbrt(zeta_threshold));
            let t14 = (simd::cbrt(t10));
            let t16 = (((t10).simd_le(zeta_threshold)).select(t12 * zeta_threshold, t14 * t10));
            let t17 = f64x8::splat(1.0) / t4 * t16;
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t20 * t20;
            let t22 = v_sigma * t21;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t27 = t22 * t26;
            let t29 = v_tau * t21;
            let t31 = f64x8::splat(1.0) / t24 / v_rho;
            let t32 = t29 * t31;
            let t34 = f64x8::splat(M_CBRT6);
            let t35 = t34 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = t35 * t38;
            let t41 = f64x8::splat(1.0) + f64x8::splat(0.00186726) * t27 + f64x8::splat(0.00373452) * t32 - f64x8::splat(0.001120356) * t39;
            let t47 = -f64x8::splat(0.003556788) * t27 + f64x8::splat(0.012500652) * t32 - f64x8::splat(0.0037501956) * t39;
            let t48 = t41 * t41;
            let t49 = f64x8::splat(1.0) / t48;
            let t51 = v_sigma * v_sigma;
            let t52 = t51 * t20;
            let t53 = t23 * t23;
            let t54 = t53 * v_rho;
            let t56 = f64x8::splat(1.0) / t18 / t54;
            let t61 = f64x8::splat(2.0) * t32 - f64x8::splat(3.0) / f64x8::splat(5.0) * t39;
            let t65 = t61 * t61;
            let t67 = -f64x8::splat(4.709036e-05) * t52 * t56 - f64x8::splat(0.0001282732) * t22 * t26 * t61 + f64x8::splat(0.0003574822) * t65;
            let t68 = t48 * t41;
            let t69 = f64x8::splat(1.0) / t68;
            let t73 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = f64x8::splat(M_CBRT4);
            let t77 = (-f64x8::splat(0.9800683) / t41 + t47 * t49 + t67 * t69) * t74 * t76;
            let t80 = ((t3).select(f64x8::splat(0.0), t19 * t77 / f64x8::splat(4.0)));
            let tzk0 = f64x8::splat(2.0) * t80;
            acc_zk = tzk0;
            let t82 = t17 / t24;
            let t85 = t23 * v_rho;
            let t87 = f64x8::splat(1.0) / t24 / t85;
            let t88 = t22 * t87;
            let t90 = t29 * t26;
            let t92 = -f64x8::splat(0.00497936) * t88 - f64x8::splat(0.0062242) * t90;
            let t97 = f64x8::splat(0.009484768) * t88 - f64x8::splat(0.02083442) * t90;
            let t99 = t47 * t69;
            let t102 = t53 * t23;
            let t104 = f64x8::splat(1.0) / t18 / t102;
            let t110 = v_sigma * t20;
            let t114 = t61 * v_tau;
            let t115 = t21 * t26;
            let t118 = f64x8::splat(0.00025114858666666666) * t52 * t104 + f64x8::splat(0.00034206186666666666) * t22 * t87 * t61 + f64x8::splat(0.0008551546666666666) * t110 * t56 * v_tau - f64x8::splat(0.0023832146666666666) * t114 * t115;
            let t120 = t48 * t48;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t67 * t121;
            let t127 = (f64x8::splat(0.9800683) * t49 * t92 + t97 * t49 - f64x8::splat(2.0) * t99 * t92 + t118 * t69 - f64x8::splat(3.0) * t122 * t92) * t74 * t76;
            let t131 = ((t3).select(f64x8::splat(0.0), t82 * t77 / f64x8::splat(12.0) + t19 * t127 / f64x8::splat(4.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t131 + f64x8::splat(2.0) * t80;
            acc_vrho = tvrho0;
            let t134 = t49 * t21;
            let t135 = t134 * t26;
            let t137 = t99 * t115;
            let t139 = t110 * t56;
            let t141 = t115 * t61;
            let t143 = -f64x8::splat(9.418072e-05) * t139 - f64x8::splat(0.0001282732) * t141;
            let t145 = t122 * t115;
            let t149 = (-f64x8::splat(0.001726745666142) * t135 - f64x8::splat(0.00373452) * t137 + t143 * t69 - f64x8::splat(0.00560178) * t145) * t74 * t76;
            let t152 = ((t3).select(f64x8::splat(0.0), t19 * t149 / f64x8::splat(4.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t152;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t156 = t21 * t31;
            let t160 = f64x8::splat(1.0) / t18 / t53;
            let t166 = -f64x8::splat(0.0005130928) * t110 * t160 + f64x8::splat(0.0014299288) * t61 * t21 * t31;
            let t172 = (f64x8::splat(0.016160736667716) * t134 * t31 - f64x8::splat(0.00746904) * t99 * t156 + t166 * t69 - f64x8::splat(0.01120356) * t122 * t156) * t74 * t76;
            let t175 = ((t3).select(f64x8::splat(0.0), t19 * t172 / f64x8::splat(4.0)));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t175;
            acc_vtau = tvtau0;
            let t178 = t17 * t31;
            let t183 = t92 * t92;
            let t187 = f64x8::splat(1.0) / t24 / t53;
            let t188 = t22 * t187;
            let t190 = t29 * t87;
            let t192 = f64x8::splat(0.018257653333333332) * t188 + f64x8::splat(0.016597866666666666) * t190;
            let t197 = -f64x8::splat(0.034777482666666665) * t188 + f64x8::splat(0.055558453333333334) * t190;
            let t199 = t97 * t69;
            let t202 = t47 * t121;
            let t207 = t53 * t85;
            let t209 = f64x8::splat(1.0) / t18 / t207;
            let t218 = v_tau * v_tau;
            let t219 = t218 * t20;
            let t222 = t21 * t87;
            let t225 = -f64x8::splat(0.0015906077155555555) * t52 * t209 - f64x8::splat(0.0012542268444444445) * t22 * t187 * t61 - f64x8::splat(0.006841237333333333) * t110 * t104 * v_tau + f64x8::splat(0.015888097777777777) * t219 * t56 + f64x8::splat(0.006355239111111111) * t114 * t222;
            let t227 = t118 * t121;
            let t231 = f64x8::splat(1.0) / t120 / t41;
            let t232 = t67 * t231;
            let t239 = (-f64x8::splat(1.9601366) * t69 * t183 + f64x8::splat(0.9800683) * t49 * t192 + t197 * t49 - f64x8::splat(4.0) * t199 * t92 + f64x8::splat(6.0) * t202 * t183 - f64x8::splat(2.0) * t99 * t192 + t225 * t69 - f64x8::splat(6.0) * t227 * t92 + f64x8::splat(12.0) * t232 * t183 - f64x8::splat(3.0) * t122 * t192) * t74 * t76;
            let t243 = ((t3).select(f64x8::splat(0.0), -t178 * t77 / f64x8::splat(18.0) + t82 * t127 / f64x8::splat(6.0) + t19 * t239 / f64x8::splat(4.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t243 + f64x8::splat(4.0) * t131;
            acc_v2rho2 = tv2rho20;
            let t248 = t69 * t21;
            let t250 = t248 * t26 * t92;
            let t252 = t134 * t87;
            let t254 = t199 * t115;
            let t256 = t115 * t92;
            let t257 = t202 * t256;
            let t259 = t99 * t222;
            let t261 = t110 * t104;
            let t263 = t222 * t61;
            let t265 = t20 * t56;
            let t266 = t265 * v_tau;
            let t268 = f64x8::splat(0.0005022971733333333) * t261 + f64x8::splat(0.00034206186666666666) * t263 + f64x8::splat(0.0008551546666666666) * t266;
            let t270 = t143 * t121;
            let t273 = t227 * t115;
            let t275 = t232 * t256;
            let t277 = t122 * t222;
            let t281 = (f64x8::splat(0.003453491332284) * t250 + f64x8::splat(0.004604655109712) * t252 - f64x8::splat(0.00373452) * t254 + f64x8::splat(0.01120356) * t257 + f64x8::splat(0.00995872) * t259 + t268 * t69 - f64x8::splat(3.0) * t270 * t92 - f64x8::splat(0.00560178) * t273 + f64x8::splat(0.02240712) * t275 + f64x8::splat(0.01493808) * t277) * t74 * t76;
            let t285 = ((t3).select(f64x8::splat(0.0), t82 * t149 / f64x8::splat(12.0) + t19 * t281 / f64x8::splat(4.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t285 + f64x8::splat(2.0) * t152;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t290 = t31 * t92;
            let t296 = t156 * t92;
            let t305 = f64x8::splat(0.0022234021333333333) * t139 - f64x8::splat(0.009532858666666666) * v_tau * t20 * t160 - f64x8::splat(0.0023832146666666666) * t141;
            let t307 = t166 * t121;
            let t317 = (-f64x8::splat(0.032321473335432) * t248 * t290 - f64x8::splat(0.02693456111286) * t135 - f64x8::splat(0.00746904) * t199 * t156 + f64x8::splat(0.02240712) * t202 * t296 + f64x8::splat(0.0124484) * t137 + t305 * t69 - f64x8::splat(3.0) * t307 * t92 - f64x8::splat(0.01120356) * t227 * t156 + f64x8::splat(0.04481424) * t232 * t296 + f64x8::splat(0.0186726) * t145) * t74 * t76;
            let t321 = ((t3).select(f64x8::splat(0.0), t82 * t172 / f64x8::splat(12.0) + t19 * t317 / f64x8::splat(4.0)));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t321 + f64x8::splat(2.0) * t175;
            acc_v2rhotau = tv2rhotau0;
            let t324 = t69 * t20;
            let t325 = t324 * t56;
            let t327 = t202 * t265;
            let t329 = t270 * t115;
            let t331 = t232 * t265;
            let t335 = (-f64x8::splat(5.471779570623876e-05) * t325 + f64x8::splat(4.18399188912e-05) * t327 - f64x8::splat(0.01120356) * t329 + f64x8::splat(8.36798377824e-05) * t331) * t74 * t76;
            let t338 = ((t3).select(f64x8::splat(0.0), t19 * t335 / f64x8::splat(4.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t338;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t340 = t324 * t160;
            let t342 = t20 * t160;
            let t343 = t202 * t342;
            let t345 = t307 * t115;
            let t349 = t232 * t342;
            let t353 = (-f64x8::splat(0.0005806664049135975) * t340 + f64x8::splat(8.36798377824e-05) * t343 - f64x8::splat(0.00560178) * t345 - f64x8::splat(0.01120356) * t270 * t156 + f64x8::splat(0.0001673596755648) * t349) * t74 * t76;
            let t356 = ((t3).select(f64x8::splat(0.0), t19 * t353 / f64x8::splat(4.0)));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t356;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t359 = f64x8::splat(1.0) / t18 / t85;
            let t362 = t20 * t359;
            let t371 = (f64x8::splat(0.005291569083170565) * t324 * t359 + f64x8::splat(0.0001673596755648) * t202 * t362 - f64x8::splat(0.02240712) * t307 * t156 + f64x8::splat(0.0003347193511296) * t232 * t362) * t74 * t76;
            let t374 = ((t3).select(f64x8::splat(0.0), t19 * t371 / f64x8::splat(4.0)));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t374;
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
