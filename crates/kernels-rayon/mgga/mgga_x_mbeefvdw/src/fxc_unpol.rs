//! MGGA_X_MBEEFVDW fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeefvdw.c`
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
pub fn mgga_x_mbeefvdw_fxc_unpol(
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
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = t26 * v_sigma;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = v_sigma * t29;
            let t36 = t35 * t33;
            let t39 = f64x8::splat(6.5124) + t26 * t36 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t34 * t40;
            let t42 = t27 * t41;
            let t44 = t42 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t45 = v_tau * t29;
            let t47 = f64x8::splat(1.0) / t31 / v_rho;
            let t53 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t45 * t47 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t54 = (f64x8::splat(10000.0)).simd_le(t53);
            let t55 = (f64x8::splat(10000.0)).simd_lt(t53);
            let t56 = ((t55).select(t53, f64x8::splat(10000.0)));
            let t57 = t56 * t56;
            let t60 = t57 * t56;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t57 * t57;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = ((t55).select(f64x8::splat(10000.0), t53));
            let t67 = t66 * t66;
            let t68 = f64x8::splat(1.0) - t67;
            let t69 = t68 * t68;
            let t70 = t69 * t68;
            let t71 = t67 * t66;
            let t72 = f64x8::splat(1.0) + t71;
            let t74 = t71 * t72 + f64x8::splat(1.0);
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = ((t54).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t57 - t61 + f64x8::splat(3.0) * t63, -t70 * t75));
            let t78 = t77 * t77;
            let t79 = t78 * t78;
            let t82 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t79 - f64x8::splat(15.0) / f64x8::splat(4.0) * t78;
            let t85 = t78 * t77;
            let t88 = f64x8::splat(5.0) / f64x8::splat(2.0) * t85 - f64x8::splat(3.0) / f64x8::splat(2.0) * t77;
            let t92 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t78;
            let t95 = t44 * t77;
            let t99 = t44 * t44;
            let t100 = t99 * t99;
            let t106 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t100 - f64x8::splat(15.0) / f64x8::splat(4.0) * t99;
            let t113 = -f64x8::splat(1.00478906e-07) * t44 * t82 - f64x8::splat(0.00608338264) * t44 * t88 + f64x8::splat(0.0318024096) * t44 * t92 + f64x8::splat(0.0453837246) * t95 - f64x8::splat(0.06972770593) * t77 + f64x8::splat(0.0217681859775) * t78 + f64x8::splat(0.00618699843125) * t100 + f64x8::splat(0.01214700985) * t42 - f64x8::splat(0.0851282539125) * t99 - f64x8::splat(3.40722258e-09) * t106 * t82 + f64x8::splat(5.74317889e-08) * t106 * t88 - f64x8::splat(5.00749348e-07) * t106 * t92;
            let t114 = t106 * t77;
            let t116 = t99 * t44;
            let t119 = f64x8::splat(5.0) / f64x8::splat(2.0) * t116 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t126 = t119 * t77;
            let t129 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t99;
            let t136 = t129 * t77;
            let t141 = f64x8::splat(1.0451438955835) + f64x8::splat(9.19317034e-07) * t114 + f64x8::splat(3.97324768e-09) * t119 * t82 - f64x8::splat(5.49909413e-08) * t119 * t88 + f64x8::splat(1.33707403e-07) * t119 * t92 + f64x8::splat(0.0192374554) * t126 + f64x8::splat(2.01895739e-07) * t129 * t82 - f64x8::splat(6.57949254e-07) * t129 * t88 - f64x8::splat(0.00521818079) * t129 * t92 - f64x8::splat(0.0222650139) * t136 + f64x8::splat(0.00061919587625) * t79 - f64x8::splat(0.050282912) * t116 + f64x8::splat(0.00351985355) * t85;
            let t142 = t113 + t141;
            let t146 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t142));
            let tzk0 = f64x8::splat(2.0) * t146;
            acc_zk = tzk0;
            let t148 = t18 / t31;
            let t154 = t30 * v_rho;
            let t156 = f64x8::splat(1.0) / t31 / t154;
            let t162 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * t45 * t33 + t35 * t156 / f64x8::splat(3.0)) * t21 * t25;
            let t163 = ((t55).select(t162, f64x8::splat(0.0)));
            let t166 = t63 * t163;
            let t169 = f64x8::splat(1.0) / t62 / t56;
            let t170 = t169 * t163;
            let t173 = t69 * t75;
            let t174 = ((t55).select(f64x8::splat(0.0), t162));
            let t175 = t66 * t174;
            let t178 = t74 * t74;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t70 * t179;
            let t181 = t67 * t72;
            let t183 = t67 * t67;
            let t184 = t183 * t66;
            let t187 = f64x8::splat(3.0) * t181 * t174 + f64x8::splat(3.0) * t184 * t174;
            let t190 = ((t54).select(f64x8::splat(6.0) * t61 * t163 + f64x8::splat(3.0) * t166 - f64x8::splat(12.0) * t170, f64x8::splat(6.0) * t173 * t175 + t180 * t187));
            let t192 = t29 * t156;
            let t193 = t192 * t40;
            let t194 = t27 * t193;
            let t196 = t21 * t21;
            let t198 = f64x8::splat(1.0) / t23 / t22;
            let t199 = t196 * t198;
            let t200 = v_sigma * v_sigma;
            let t201 = t199 * t200;
            let t202 = t30 * t30;
            let t203 = t202 * t30;
            let t205 = f64x8::splat(1.0) / t19 / t203;
            let t207 = t39 * t39;
            let t208 = f64x8::splat(1.0) / t207;
            let t209 = t28 * t205 * t208;
            let t210 = t201 * t209;
            let t216 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t194 + t210 / f64x8::splat(54.0);
            let t217 = t44 * t216;
            let t232 = t216 * t92;
            let t234 = t216 * t77;
            let t236 = t44 * t190;
            let t238 = t216 * t82;
            let t240 = t85 * t190;
            let t242 = t77 * t190;
            let t246 = f64x8::splat(35.0) / f64x8::splat(2.0) * t240 - f64x8::splat(15.0) / f64x8::splat(2.0) * t242;
            let t249 = t216 * t88;
            let t251 = t78 * t190;
            let t253 = -f64x8::splat(0.06972770593) * t190 - f64x8::splat(0.032392026266666665) * t194 + f64x8::splat(0.0026993355222222223) * t210 + f64x8::splat(4.01122209e-07) * t126 * t190 + f64x8::splat(6.05687217e-07) * t217 * t82 - f64x8::splat(1.973847762e-06) * t217 * t88 - f64x8::splat(0.01565454237) * t217 * t92 - f64x8::splat(0.01565454237) * t136 * t190 - f64x8::splat(0.0667950417) * t217 * t77 + f64x8::splat(0.0954072288) * t95 * t190 - f64x8::splat(1.502248044e-06) * t114 * t190 + f64x8::splat(0.0318024096) * t232 + f64x8::splat(0.0453837246) * t234 + f64x8::splat(0.0453837246) * t236 - f64x8::splat(1.00478906e-07) * t238 + f64x8::splat(0.002476783505) * t240 + f64x8::splat(0.043536371955) * t242 - f64x8::splat(1.00478906e-07) * t44 * t246 - f64x8::splat(0.00608338264) * t249 + f64x8::splat(0.01055956065) * t251;
            let t256 = f64x8::splat(15.0) / f64x8::splat(2.0) * t251 - f64x8::splat(3.0) / f64x8::splat(2.0) * t190;
            let t261 = t129 * t190;
            let t263 = t99 * t216;
            let t268 = f64x8::splat(15.0) / f64x8::splat(2.0) * t263 + t194 / f64x8::splat(3.0) - t210 / f64x8::splat(36.0);
            let t271 = t268 * t77;
            let t273 = t119 * t190;
            let t279 = t116 * t216;
            let t282 = f64x8::splat(35.0) / f64x8::splat(2.0) * t279 - f64x8::splat(15.0) / f64x8::splat(2.0) * t217;
            let t300 = t282 * t77;
            let t302 = t106 * t190;
            let t306 = f64x8::splat(3.97324768e-09) * t268 * t82 + f64x8::splat(3.97324768e-09) * t119 * t246 - f64x8::splat(5.49909413e-08) * t268 * t88 - f64x8::splat(5.49909413e-08) * t119 * t256 + f64x8::splat(5.74317889e-08) * t106 * t256 + f64x8::splat(0.024747993725) * t279 - f64x8::splat(0.170256507825) * t217 - f64x8::splat(5.00749348e-07) * t282 * t92 + f64x8::splat(9.19317034e-07) * t300 + f64x8::splat(9.19317034e-07) * t302 - f64x8::splat(3.40722258e-09) * t282 * t82;
            let t308 = t253 - f64x8::splat(0.00608338264) * t44 * t256 - f64x8::splat(6.57949254e-07) * t129 * t256 - f64x8::splat(0.0222650139) * t261 - f64x8::splat(0.150848736) * t263 + f64x8::splat(1.33707403e-07) * t268 * t92 + f64x8::splat(0.0192374554) * t271 + f64x8::splat(0.0192374554) * t273 + f64x8::splat(2.01895739e-07) * t129 * t246 - f64x8::splat(3.40722258e-09) * t106 * t246 + f64x8::splat(5.74317889e-08) * t282 * t88 + t306;
            let t313 = ((t3).select(f64x8::splat(0.0), -t7 * t148 * t142 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t308));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t313 + f64x8::splat(2.0) * t146;
            acc_vrho = tvrho0;
            let t316 = t26 * t41;
            let t319 = t202 * v_rho;
            let t323 = t28 / t19 / t319 * t208;
            let t324 = t199 * v_sigma * t323;
            let t326 = t316 / f64x8::splat(12.0) - t324 / f64x8::splat(144.0);
            let t327 = t326 * t92;
            let t329 = t326 * t77;
            let t331 = t26 * t34;
            let t332 = f64x8::splat(5.0) / f64x8::splat(72.0) * t331;
            let t333 = ((t55).select(-t332, f64x8::splat(0.0)));
            let t336 = t63 * t333;
            let t338 = t169 * t333;
            let t341 = ((t55).select(f64x8::splat(0.0), -t332));
            let t342 = t66 * t341;
            let t348 = f64x8::splat(3.0) * t181 * t341 + f64x8::splat(3.0) * t184 * t341;
            let t351 = ((t54).select(f64x8::splat(6.0) * t61 * t333 + f64x8::splat(3.0) * t336 - f64x8::splat(12.0) * t338, f64x8::splat(6.0) * t173 * t342 + t180 * t348));
            let t352 = t44 * t351;
            let t354 = t326 * t82;
            let t356 = t85 * t351;
            let t358 = t77 * t351;
            let t362 = f64x8::splat(35.0) / f64x8::splat(2.0) * t356 - f64x8::splat(15.0) / f64x8::splat(2.0) * t358;
            let t365 = t326 * t88;
            let t367 = t78 * t351;
            let t371 = f64x8::splat(15.0) / f64x8::splat(2.0) * t367 - f64x8::splat(3.0) / f64x8::splat(2.0) * t351;
            let t374 = t129 * t351;
            let t376 = t119 * t351;
            let t382 = t99 * t326;
            let t387 = f64x8::splat(15.0) / f64x8::splat(2.0) * t382 - t316 / f64x8::splat(8.0) + t324 / f64x8::splat(96.0);
            let t390 = t387 * t77;
            let t397 = f64x8::splat(0.0318024096) * t327 + f64x8::splat(0.0453837246) * t329 + f64x8::splat(0.0453837246) * t352 - f64x8::splat(1.00478906e-07) * t354 + f64x8::splat(0.002476783505) * t356 + f64x8::splat(0.043536371955) * t358 - f64x8::splat(1.00478906e-07) * t44 * t362 - f64x8::splat(0.00608338264) * t365 + f64x8::splat(0.01055956065) * t367 - f64x8::splat(0.00608338264) * t44 * t371 - f64x8::splat(0.0222650139) * t374 + f64x8::splat(0.0192374554) * t376 + f64x8::splat(2.01895739e-07) * t129 * t362 - f64x8::splat(6.57949254e-07) * t129 * t371 - f64x8::splat(0.150848736) * t382 + f64x8::splat(1.33707403e-07) * t387 * t92 + f64x8::splat(0.0192374554) * t390 + f64x8::splat(0.01214700985) * t316 + f64x8::splat(3.97324768e-09) * t387 * t82 + f64x8::splat(3.97324768e-09) * t119 * t362;
            let t402 = t116 * t326;
            let t404 = t44 * t326;
            let t408 = f64x8::splat(35.0) / f64x8::splat(2.0) * t402 - f64x8::splat(15.0) / f64x8::splat(2.0) * t404;
            let t437 = t408 * t77;
            let t439 = t106 * t351;
            let t442 = -f64x8::splat(0.01565454237) * t404 * t92 - f64x8::splat(0.01565454237) * t136 * t351 - f64x8::splat(0.0667950417) * t404 * t77 + f64x8::splat(0.0954072288) * t95 * t351 - f64x8::splat(1.502248044e-06) * t114 * t351 + f64x8::splat(4.01122209e-07) * t126 * t351 + f64x8::splat(5.74317889e-08) * t106 * t371 - f64x8::splat(5.00749348e-07) * t408 * t92 + f64x8::splat(9.19317034e-07) * t437 + f64x8::splat(9.19317034e-07) * t439 - f64x8::splat(0.0010122508208333333) * t324;
            let t444 = t397 - f64x8::splat(5.49909413e-08) * t387 * t88 - f64x8::splat(5.49909413e-08) * t119 * t371 + f64x8::splat(0.024747993725) * t402 - f64x8::splat(0.170256507825) * t404 - f64x8::splat(3.40722258e-09) * t408 * t82 - f64x8::splat(3.40722258e-09) * t106 * t362 + f64x8::splat(5.74317889e-08) * t408 * t88 - f64x8::splat(0.06972770593) * t351 + f64x8::splat(6.05687217e-07) * t404 * t82 - f64x8::splat(1.973847762e-06) * t404 * t88 + t442;
            let t448 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t444));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t448;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t452 = f64x8::splat(5.0) / f64x8::splat(9.0) * t29 * t47 * t26;
            let t453 = ((t55).select(t452, f64x8::splat(0.0)));
            let t456 = t63 * t453;
            let t458 = t169 * t453;
            let t461 = ((t55).select(f64x8::splat(0.0), t452));
            let t462 = t66 * t461;
            let t468 = f64x8::splat(3.0) * t181 * t461 + f64x8::splat(3.0) * t184 * t461;
            let t471 = ((t54).select(f64x8::splat(6.0) * t61 * t453 + f64x8::splat(3.0) * t456 - f64x8::splat(12.0) * t458, f64x8::splat(6.0) * t173 * t462 + t180 * t468));
            let t472 = t85 * t471;
            let t474 = t77 * t471;
            let t476 = f64x8::splat(35.0) / f64x8::splat(2.0) * t472 - f64x8::splat(15.0) / f64x8::splat(2.0) * t474;
            let t479 = t78 * t471;
            let t482 = f64x8::splat(15.0) / f64x8::splat(2.0) * t479 - f64x8::splat(3.0) / f64x8::splat(2.0) * t471;
            let t487 = t106 * t471;
            let t495 = t119 * t471;
            let t503 = t129 * t471;
            let t511 = t44 * t471;
            let t517 = -f64x8::splat(3.40722258e-09) * t106 * t476 + f64x8::splat(5.74317889e-08) * t106 * t482 - f64x8::splat(1.502248044e-06) * t114 * t471 + f64x8::splat(9.19317034e-07) * t487 + f64x8::splat(3.97324768e-09) * t119 * t476 - f64x8::splat(5.49909413e-08) * t119 * t482 + f64x8::splat(4.01122209e-07) * t126 * t471 + f64x8::splat(0.0192374554) * t495 + f64x8::splat(2.01895739e-07) * t129 * t476 - f64x8::splat(6.57949254e-07) * t129 * t482 - f64x8::splat(0.01565454237) * t136 * t471 - f64x8::splat(0.0222650139) * t503 - f64x8::splat(1.00478906e-07) * t44 * t476 - f64x8::splat(0.00608338264) * t44 * t482 + f64x8::splat(0.0954072288) * t95 * t471 + f64x8::splat(0.0453837246) * t511 + f64x8::splat(0.002476783505) * t472 + f64x8::splat(0.01055956065) * t479 + f64x8::splat(0.043536371955) * t474 - f64x8::splat(0.06972770593) * t471;
            let t521 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t517));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t521;
            acc_vtau = tvtau0;
            let t524 = t18 * t47;
            let t533 = t163 * t163;
            let t539 = f64x8::splat(1.0) / t31 / t202;
            let t545 = f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(40.0) / f64x8::splat(9.0) * t45 * t156 - f64x8::splat(11.0) / f64x8::splat(9.0) * t35 * t539) * t21 * t25;
            let t546 = ((t55).select(t545, f64x8::splat(0.0)));
            let t549 = t169 * t533;
            let t554 = f64x8::splat(1.0) / t62 / t57;
            let t555 = t554 * t533;
            let t560 = t68 * t75;
            let t561 = t174 * t174;
            let t562 = t67 * t561;
            let t565 = t69 * t179;
            let t566 = t175 * t187;
            let t571 = ((t55).select(f64x8::splat(0.0), t545));
            let t572 = t66 * t571;
            let t576 = f64x8::splat(1.0) / t178 / t74;
            let t577 = t70 * t576;
            let t578 = t187 * t187;
            let t581 = t66 * t72;
            let t590 = f64x8::splat(3.0) * t181 * t571 + f64x8::splat(24.0) * t183 * t561 + f64x8::splat(3.0) * t184 * t571 + f64x8::splat(6.0) * t581 * t561;
            let t593 = ((t54).select(-f64x8::splat(12.0) * t169 * t546 - f64x8::splat(18.0) * t63 * t533 + f64x8::splat(6.0) * t61 * t546 + f64x8::splat(3.0) * t63 * t546 - f64x8::splat(12.0) * t549 + f64x8::splat(60.0) * t555, f64x8::splat(6.0) * t173 * t561 + f64x8::splat(6.0) * t173 * t572 + t180 * t590 - f64x8::splat(24.0) * t560 * t562 - f64x8::splat(12.0) * t565 * t566 - f64x8::splat(2.0) * t577 * t578));
            let t596 = t22 * t22;
            let t597 = f64x8::splat(1.0) / t596;
            let t599 = t597 * t200 * v_sigma;
            let t600 = t202 * t202;
            let t601 = t600 * t30;
            let t602 = f64x8::splat(1.0) / t601;
            let t604 = f64x8::splat(1.0) / t207 / t39;
            let t605 = t602 * t604;
            let t606 = t599 * t605;
            let t610 = t216 * t216;
            let t612 = t190 * t190;
            let t615 = t29 * t539;
            let t616 = t615 * t40;
            let t617 = t27 * t616;
            let t619 = t202 * t154;
            let t621 = f64x8::splat(1.0) / t19 / t619;
            let t623 = t28 * t621 * t208;
            let t624 = t201 * t623;
            let t629 = f64x8::splat(22.0) / f64x8::splat(27.0) * t617 - t624 / f64x8::splat(6.0) + f64x8::splat(4.0) / f64x8::splat(81.0) * t606;
            let t630 = t44 * t629;
            let t649 = f64x8::splat(8.02244418e-07) * t271 * t190 + f64x8::splat(4.01122209e-07) * t126 * t593 + f64x8::splat(0.007198228059259259) * t606 - f64x8::splat(0.09392725422) * t217 * t242 - f64x8::splat(0.170256507825) * t610 + f64x8::splat(0.043536371955) * t612 - f64x8::splat(0.06972770593) * t593 + f64x8::splat(0.11877076297777778) * t617 - f64x8::splat(0.0242940197) * t624 - f64x8::splat(0.0667950417) * t630 * t77 - f64x8::splat(0.1335900834) * t217 * t190 + f64x8::splat(0.1908144576) * t234 * t190 + f64x8::splat(0.0954072288) * t95 * t593 - f64x8::splat(3.004496088e-06) * t300 * t190 - f64x8::splat(1.502248044e-06) * t114 * t593 + f64x8::splat(6.05687217e-07) * t630 * t82 + f64x8::splat(1.211374434e-06) * t217 * t246 - f64x8::splat(1.973847762e-06) * t630 * t88;
            let t656 = t77 * t612;
            let t658 = t78 * t593;
            let t663 = f64x8::splat(15.0) * t656 + f64x8::splat(15.0) / f64x8::splat(2.0) * t658 - f64x8::splat(3.0) / f64x8::splat(2.0) * t593;
            let t668 = t129 * t593;
            let t670 = t44 * t610;
            let t672 = t99 * t629;
            let t679 = f64x8::splat(15.0) * t670 + f64x8::splat(15.0) / f64x8::splat(2.0) * t672 - f64x8::splat(11.0) / f64x8::splat(9.0) * t617 + t624 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t606;
            let t682 = t679 * t77;
            let t684 = t268 * t190;
            let t686 = t119 * t593;
            let t688 = t78 * t612;
            let t690 = t85 * t593;
            let t692 = t77 * t593;
            let t698 = f64x8::splat(105.0) / f64x8::splat(2.0) * t688 + f64x8::splat(35.0) / f64x8::splat(2.0) * t690 - f64x8::splat(15.0) / f64x8::splat(2.0) * t612 - f64x8::splat(15.0) / f64x8::splat(2.0) * t692;
            let t701 = -f64x8::splat(3.947695524e-06) * t217 * t256 - f64x8::splat(0.01565454237) * t630 * t92 - f64x8::splat(0.01565454237) * t136 * t593 + f64x8::splat(0.0211191213) * t656 + f64x8::splat(0.01055956065) * t658 - f64x8::splat(0.00608338264) * t44 * t663 - f64x8::splat(6.57949254e-07) * t129 * t663 - f64x8::splat(0.0222650139) * t668 - f64x8::splat(0.301697472) * t670 - f64x8::splat(0.150848736) * t672 + f64x8::splat(1.33707403e-07) * t679 * t92 + f64x8::splat(0.0192374554) * t682 + f64x8::splat(0.0384749108) * t684 + f64x8::splat(0.0192374554) * t686 + f64x8::splat(0.007430350515) * t688 + f64x8::splat(0.002476783505) * t690 + f64x8::splat(0.043536371955) * t692 + f64x8::splat(2.01895739e-07) * t129 * t698;
            let t719 = t99 * t610;
            let t721 = t116 * t629;
            let t728 = f64x8::splat(105.0) / f64x8::splat(2.0) * t719 + f64x8::splat(35.0) / f64x8::splat(2.0) * t721 - f64x8::splat(15.0) / f64x8::splat(2.0) * t610 - f64x8::splat(15.0) / f64x8::splat(2.0) * t630;
            let t731 = t728 * t77;
            let t733 = t282 * t190;
            let t735 = t106 * t593;
            let t743 = f64x8::splat(3.97324768e-09) * t679 * t82 + f64x8::splat(7.94649536e-09) * t268 * t246 + f64x8::splat(3.97324768e-09) * t119 * t698 - f64x8::splat(5.49909413e-08) * t679 * t88 - f64x8::splat(1.099818826e-07) * t268 * t256 - f64x8::splat(5.49909413e-08) * t119 * t663 + f64x8::splat(1.148635778e-07) * t282 * t256 + f64x8::splat(5.74317889e-08) * t106 * t663 + f64x8::splat(0.074243981175) * t719 + f64x8::splat(0.024747993725) * t721 - f64x8::splat(0.170256507825) * t630 - f64x8::splat(5.00749348e-07) * t728 * t92 + f64x8::splat(9.19317034e-07) * t731 + f64x8::splat(1.838634068e-06) * t733 + f64x8::splat(9.19317034e-07) * t735 - f64x8::splat(3.40722258e-09) * t728 * t82 - f64x8::splat(6.81444516e-09) * t282 * t246 - f64x8::splat(3.40722258e-09) * t106 * t698;
            let t748 = t216 * t256;
            let t752 = t629 * t77;
            let t754 = t216 * t190;
            let t756 = t44 * t593;
            let t760 = t216 * t246;
            let t764 = t610 * t77;
            let t780 = f64x8::splat(5.74317889e-08) * t728 * t88 - f64x8::splat(0.00608338264) * t629 * t88 - f64x8::splat(0.01216676528) * t748 - f64x8::splat(1.00478906e-07) * t44 * t698 + f64x8::splat(0.0453837246) * t752 + f64x8::splat(0.0907674492) * t754 + f64x8::splat(0.0453837246) * t756 - f64x8::splat(1.00478906e-07) * t629 * t82 - f64x8::splat(2.00957812e-07) * t760 + f64x8::splat(0.0318024096) * t629 * t92 - f64x8::splat(0.0667950417) * t764 + f64x8::splat(0.0954072288) * t44 * t612 - f64x8::splat(1.502248044e-06) * t106 * t612 + f64x8::splat(6.05687217e-07) * t610 * t82 - f64x8::splat(1.973847762e-06) * t610 * t88 - f64x8::splat(0.01565454237) * t610 * t92 - f64x8::splat(0.01565454237) * t129 * t612 + f64x8::splat(4.01122209e-07) * t119 * t612;
            let t782 = t649 + t701 + t743 + t780;
            let t787 = ((t3).select(f64x8::splat(0.0), t7 * t524 * t142 / f64x8::splat(12.0) - t7 * t148 * t308 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t782));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t787 + f64x8::splat(4.0) * t313;
            acc_v2rho2 = tv2rho20;
            let t793 = t358 * t190;
            let t795 = t367 * t190;
            let t797 = t597 * t200;
            let t798 = t600 * v_rho;
            let t799 = f64x8::splat(1.0) / t798;
            let t801 = t797 * t799 * t604;
            let t815 = t26 * t192;
            let t816 = f64x8::splat(5.0) / f64x8::splat(27.0) * t815;
            let t817 = ((t55).select(t816, f64x8::splat(0.0)));
            let t822 = t63 * t817;
            let t824 = t554 * t333;
            let t827 = t169 * t817;
            let t830 = t67 * t341;
            let t831 = t830 * t174;
            let t834 = t342 * t187;
            let t837 = t174 * t341;
            let t840 = ((t55).select(f64x8::splat(0.0), t816));
            let t841 = t66 * t840;
            let t844 = t348 * t66;
            let t845 = t844 * t174;
            let t848 = t348 * t187;
            let t853 = t183 * t174;
            let t860 = f64x8::splat(3.0) * t181 * t840 + f64x8::splat(3.0) * t184 * t840 + f64x8::splat(24.0) * t853 * t341 + f64x8::splat(6.0) * t581 * t837;
            let t863 = ((t54).select(-f64x8::splat(18.0) * t336 * t163 - f64x8::splat(12.0) * t338 * t163 + f64x8::splat(60.0) * t824 * t163 + f64x8::splat(6.0) * t61 * t817 + f64x8::splat(3.0) * t822 - f64x8::splat(12.0) * t827, f64x8::splat(6.0) * t173 * t837 + f64x8::splat(6.0) * t173 * t841 + t180 * t860 - f64x8::splat(24.0) * t560 * t831 - f64x8::splat(6.0) * t565 * t834 - f64x8::splat(6.0) * t565 * t845 - f64x8::splat(2.0) * t577 * t848));
            let t870 = f64x8::splat(0.0211191213) * t793 + f64x8::splat(0.007430350515) * t795 - f64x8::splat(0.0026993355222222223) * t801 - f64x8::splat(0.04696362711) * t404 * t242 - f64x8::splat(0.04696362711) * t217 * t358 - f64x8::splat(0.0667950417) * t404 * t190 + f64x8::splat(0.0954072288) * t234 * t351 + f64x8::splat(0.0954072288) * t236 * t351 + f64x8::splat(0.0954072288) * t95 * t863 - f64x8::splat(1.502248044e-06) * t300 * t351 - f64x8::splat(1.502248044e-06) * t302 * t351;
            let t884 = t216 * t326;
            let t887 = t26 * t193;
            let t889 = t199 * t28;
            let t892 = t889 * t205 * t208 * v_sigma;
            let t895 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t887 + t892 / f64x8::splat(18.0) - t801 / f64x8::splat(54.0);
            let t896 = t44 * t895;
            let t903 = -f64x8::splat(1.502248044e-06) * t114 * t863 + f64x8::splat(4.01122209e-07) * t271 * t351 + f64x8::splat(4.01122209e-07) * t273 * t351 - f64x8::splat(0.0667950417) * t217 * t351 + f64x8::splat(4.01122209e-07) * t126 * t863 + f64x8::splat(0.0954072288) * t329 * t190 - f64x8::splat(0.06972770593) * t863 + f64x8::splat(6.05687217e-07) * t884 * t82 + f64x8::splat(6.05687217e-07) * t896 * t82 + f64x8::splat(6.05687217e-07) * t404 * t246 - f64x8::splat(1.973847762e-06) * t884 * t88;
            let t923 = t382 * t216;
            let t925 = -f64x8::splat(1.973847762e-06) * t896 * t88 - f64x8::splat(1.973847762e-06) * t404 * t256 - f64x8::splat(0.01565454237) * t884 * t92 - f64x8::splat(0.01565454237) * t896 * t92 - f64x8::splat(0.01565454237) * t261 * t351 - f64x8::splat(0.01565454237) * t136 * t863 - f64x8::splat(0.0667950417) * t884 * t77 - f64x8::splat(0.0667950417) * t896 * t77 + f64x8::splat(0.008098006566666666) * t892 - f64x8::splat(0.032392026266666665) * t887 + f64x8::splat(0.074243981175) * t923;
            let t932 = t404 * t216;
            let t936 = t78 * t863;
            let t938 = t129 * t863;
            let t940 = t268 * t351;
            let t942 = t119 * t863;
            let t944 = t408 * t190;
            let t946 = t282 * t351;
            let t948 = -f64x8::splat(1.502248044e-06) * t437 * t190 + f64x8::splat(6.05687217e-07) * t217 * t362 - f64x8::splat(1.973847762e-06) * t217 * t371 - f64x8::splat(0.301697472) * t932 + f64x8::splat(4.01122209e-07) * t390 * t190 + f64x8::splat(0.01055956065) * t936 - f64x8::splat(0.0222650139) * t938 + f64x8::splat(0.0192374554) * t940 + f64x8::splat(0.0192374554) * t942 + f64x8::splat(9.19317034e-07) * t944 + f64x8::splat(9.19317034e-07) * t946;
            let t951 = t106 * t863;
            let t953 = t85 * t863;
            let t955 = t190 * t351;
            let t957 = t77 * t863;
            let t963 = f64x8::splat(105.0) / f64x8::splat(2.0) * t795 + f64x8::splat(35.0) / f64x8::splat(2.0) * t953 - f64x8::splat(15.0) / f64x8::splat(2.0) * t955 - f64x8::splat(15.0) / f64x8::splat(2.0) * t957;
            let t969 = f64x8::splat(15.0) * t793 + f64x8::splat(15.0) / f64x8::splat(2.0) * t936 - f64x8::splat(3.0) / f64x8::splat(2.0) * t863;
            let t972 = t99 * t895;
            let t979 = f64x8::splat(15.0) * t932 + f64x8::splat(15.0) / f64x8::splat(2.0) * t972 + t887 / f64x8::splat(3.0) - t892 / f64x8::splat(12.0) + t801 / f64x8::splat(36.0);
            let t982 = t979 * t77;
            let t984 = t387 * t190;
            let t988 = f64x8::splat(9.19317034e-07) * t951 + f64x8::splat(0.002476783505) * t953 + f64x8::splat(0.043536371955) * t955 + f64x8::splat(0.043536371955) * t957 + f64x8::splat(2.01895739e-07) * t129 * t963 - f64x8::splat(6.57949254e-07) * t129 * t969 - f64x8::splat(0.150848736) * t972 + f64x8::splat(1.33707403e-07) * t979 * t92 + f64x8::splat(0.0192374554) * t982 + f64x8::splat(0.0192374554) * t984 + f64x8::splat(3.97324768e-09) * t979 * t82;
            let t1003 = t116 * t895;
            let t1011 = f64x8::splat(105.0) / f64x8::splat(2.0) * t923 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1003 - f64x8::splat(15.0) / f64x8::splat(2.0) * t884 - f64x8::splat(15.0) / f64x8::splat(2.0) * t896;
            let t1014 = f64x8::splat(3.97324768e-09) * t387 * t246 + f64x8::splat(3.97324768e-09) * t268 * t362 + f64x8::splat(3.97324768e-09) * t119 * t963 - f64x8::splat(5.49909413e-08) * t979 * t88 - f64x8::splat(5.49909413e-08) * t387 * t256 - f64x8::splat(5.49909413e-08) * t268 * t371 - f64x8::splat(5.49909413e-08) * t119 * t969 + f64x8::splat(0.024747993725) * t1003 - f64x8::splat(0.170256507825) * t884 - f64x8::splat(0.170256507825) * t896 - f64x8::splat(3.40722258e-09) * t1011 * t82;
            let t1032 = t1011 * t77;
            let t1034 = t895 * t92;
            let t1036 = t895 * t77;
            let t1038 = -f64x8::splat(3.40722258e-09) * t408 * t246 - f64x8::splat(3.40722258e-09) * t282 * t362 - f64x8::splat(3.40722258e-09) * t106 * t963 + f64x8::splat(5.74317889e-08) * t1011 * t88 + f64x8::splat(5.74317889e-08) * t408 * t256 + f64x8::splat(5.74317889e-08) * t282 * t371 + f64x8::splat(5.74317889e-08) * t106 * t969 - f64x8::splat(5.00749348e-07) * t1011 * t92 + f64x8::splat(9.19317034e-07) * t1032 + f64x8::splat(0.0318024096) * t1034 + f64x8::splat(0.0453837246) * t1036;
            let t1039 = t326 * t190;
            let t1041 = t216 * t351;
            let t1043 = t44 * t863;
            let t1045 = t895 * t88;
            let t1047 = t326 * t256;
            let t1049 = t216 * t371;
            let t1053 = t895 * t82;
            let t1055 = t326 * t246;
            let t1057 = t216 * t362;
            let t1061 = f64x8::splat(0.0453837246) * t1039 + f64x8::splat(0.0453837246) * t1041 + f64x8::splat(0.0453837246) * t1043 - f64x8::splat(0.00608338264) * t1045 - f64x8::splat(0.00608338264) * t1047 - f64x8::splat(0.00608338264) * t1049 - f64x8::splat(0.00608338264) * t44 * t969 - f64x8::splat(1.00478906e-07) * t1053 - f64x8::splat(1.00478906e-07) * t1055 - f64x8::splat(1.00478906e-07) * t1057 - f64x8::splat(1.00478906e-07) * t44 * t963;
            let t1064 = t870 + t903 + t925 + t948 + t988 + t1014 + t1038 + t1061;
            let t1069 = ((t3).select(f64x8::splat(0.0), -t7 * t148 * t444 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1064));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t1069 + f64x8::splat(2.0) * t448;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t1079 = f64x8::splat(25.0) / f64x8::splat(27.0) * t331;
            let t1080 = ((t55).select(-t1079, f64x8::splat(0.0)));
            let t1085 = t63 * t1080;
            let t1087 = t554 * t453;
            let t1090 = t169 * t1080;
            let t1093 = t67 * t461;
            let t1097 = t462 * t187;
            let t1100 = t174 * t461;
            let t1103 = ((t55).select(f64x8::splat(0.0), -t1079));
            let t1104 = t66 * t1103;
            let t1107 = t468 * t66;
            let t1111 = t468 * t187;
            let t1122 = f64x8::splat(6.0) * t581 * t1100 + f64x8::splat(3.0) * t181 * t1103 + f64x8::splat(3.0) * t184 * t1103 + f64x8::splat(24.0) * t853 * t461;
            let t1125 = ((t54).select(f64x8::splat(6.0) * t61 * t1080 + f64x8::splat(60.0) * t1087 * t163 - f64x8::splat(18.0) * t456 * t163 - f64x8::splat(12.0) * t458 * t163 + f64x8::splat(3.0) * t1085 - f64x8::splat(12.0) * t1090, -f64x8::splat(24.0) * t560 * t1093 * t174 - f64x8::splat(6.0) * t565 * t1107 * t174 - f64x8::splat(6.0) * t565 * t1097 + f64x8::splat(6.0) * t173 * t1100 + f64x8::splat(6.0) * t173 * t1104 - f64x8::splat(2.0) * t577 * t1111 + t180 * t1122));
            let t1127 = t479 * t190;
            let t1129 = t474 * t190;
            let t1162 = t85 * t1125;
            let t1164 = t190 * t471;
            let t1166 = -f64x8::splat(1.502248044e-06) * t300 * t471 - f64x8::splat(1.502248044e-06) * t302 * t471 - f64x8::splat(1.502248044e-06) * t114 * t1125 - f64x8::splat(1.973847762e-06) * t217 * t482 - f64x8::splat(0.01565454237) * t261 * t471 - f64x8::splat(0.01565454237) * t136 * t1125 - f64x8::splat(0.0667950417) * t217 * t471 + f64x8::splat(6.05687217e-07) * t217 * t476 - f64x8::splat(3.40722258e-09) * t282 * t476 + f64x8::splat(0.002476783505) * t1162 + f64x8::splat(0.043536371955) * t1164;
            let t1168 = t77 * t1125;
            let t1174 = f64x8::splat(105.0) / f64x8::splat(2.0) * t1127 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1162 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1164 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1168;
            let t1179 = t78 * t1125;
            let t1184 = f64x8::splat(15.0) * t1129 + f64x8::splat(15.0) / f64x8::splat(2.0) * t1179 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1125;
            let t1187 = t216 * t476;
            let t1191 = t216 * t482;
            let t1195 = t216 * t471;
            let t1197 = t44 * t1125;
            let t1199 = f64x8::splat(0.043536371955) * t1168 - f64x8::splat(3.40722258e-09) * t106 * t1174 + f64x8::splat(5.74317889e-08) * t282 * t482 + f64x8::splat(0.01055956065) * t1179 + f64x8::splat(5.74317889e-08) * t106 * t1184 - f64x8::splat(1.00478906e-07) * t1187 - f64x8::splat(1.00478906e-07) * t44 * t1174 - f64x8::splat(0.00608338264) * t1191 - f64x8::splat(0.00608338264) * t44 * t1184 + f64x8::splat(0.0453837246) * t1195 + f64x8::splat(0.0453837246) * t1197;
            let t1204 = t268 * t471;
            let t1206 = t119 * t1125;
            let t1210 = t282 * t471;
            let t1212 = t106 * t1125;
            let t1218 = t129 * t1125;
            let t1222 = -f64x8::splat(5.49909413e-08) * t268 * t482 - f64x8::splat(5.49909413e-08) * t119 * t1184 + f64x8::splat(0.0192374554) * t1204 + f64x8::splat(0.0192374554) * t1206 + f64x8::splat(2.01895739e-07) * t129 * t1174 + f64x8::splat(9.19317034e-07) * t1210 + f64x8::splat(9.19317034e-07) * t1212 + f64x8::splat(3.97324768e-09) * t268 * t476 + f64x8::splat(3.97324768e-09) * t119 * t1174 - f64x8::splat(0.0222650139) * t1218 - f64x8::splat(6.57949254e-07) * t129 * t1184;
            let t1224 = -f64x8::splat(0.04696362711) * t217 * t474 - f64x8::splat(0.06972770593) * t1125 + f64x8::splat(0.007430350515) * t1127 + f64x8::splat(0.0211191213) * t1129 + f64x8::splat(0.0954072288) * t234 * t471 + f64x8::splat(0.0954072288) * t236 * t471 + f64x8::splat(0.0954072288) * t95 * t1125 + f64x8::splat(4.01122209e-07) * t271 * t471 + f64x8::splat(4.01122209e-07) * t273 * t471 + f64x8::splat(4.01122209e-07) * t126 * t1125 + t1166 + t1199 + t1222;
            let t1229 = ((t3).select(f64x8::splat(0.0), -t7 * t148 * t517 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1224));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t1229 + f64x8::splat(2.0) * t521;
            acc_v2rhotau = tv2rhotau0;
            let t1232 = t597 * v_sigma;
            let t1233 = f64x8::splat(1.0) / t600;
            let t1235 = t1232 * t1233 * t604;
            let t1237 = t333 * t333;
            let t1240 = ((t55).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t1242 = f64x8::splat(6.0) * t61 * t1240;
            let t1243 = t169 * t1237;
            let t1245 = t63 * t1240;
            let t1246 = f64x8::splat(3.0) * t1245;
            let t1247 = t554 * t1237;
            let t1249 = t169 * t1240;
            let t1250 = f64x8::splat(12.0) * t1249;
            let t1252 = t341 * t341;
            let t1253 = t67 * t1252;
            let t1256 = t342 * t348;
            let t1261 = t66 * t1240;
            let t1263 = f64x8::splat(6.0) * t173 * t1261;
            let t1264 = t348 * t348;
            let t1272 = f64x8::splat(3.0) * t181 * t1240;
            let t1274 = f64x8::splat(3.0) * t184 * t1240;
            let t1275 = f64x8::splat(24.0) * t183 * t1252 + f64x8::splat(6.0) * t581 * t1252 + t1272 + t1274;
            let t1278 = ((t54).select(-f64x8::splat(18.0) * t63 * t1237 + t1242 - f64x8::splat(12.0) * t1243 + t1246 + f64x8::splat(60.0) * t1247 - t1250, f64x8::splat(6.0) * t173 * t1252 - f64x8::splat(24.0) * t560 * t1253 - f64x8::splat(12.0) * t565 * t1256 - f64x8::splat(2.0) * t577 * t1264 + t180 * t1275 + t1263));
            let t1282 = t326 * t326;
            let t1284 = t351 * t351;
            let t1300 = t326 * t371;
            let t1302 = t77 * t1284;
            let t1304 = t78 * t1278;
            let t1307 = f64x8::splat(15.0) * t1302 + f64x8::splat(15.0) / f64x8::splat(2.0) * t1304 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1278;
            let t1310 = t199 * t323;
            let t1313 = -t1310 / f64x8::splat(72.0) + t1235 / f64x8::splat(144.0);
            let t1316 = t326 * t362;
            let t1318 = t78 * t1284;
            let t1320 = t85 * t1278;
            let t1323 = t77 * t1278;
            let t1325 = f64x8::splat(105.0) / f64x8::splat(2.0) * t1318 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1320 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1284 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1323;
            let t1328 = f64x8::splat(0.0010122508208333333) * t1235 - f64x8::splat(0.06972770593) * t1278 - f64x8::splat(0.09392725422) * t404 * t358 - f64x8::splat(0.170256507825) * t1282 + f64x8::splat(0.043536371955) * t1284 - f64x8::splat(0.01565454237) * t1282 * t92 - f64x8::splat(0.01565454237) * t129 * t1284 + f64x8::splat(0.0954072288) * t44 * t1284 - f64x8::splat(1.502248044e-06) * t106 * t1284 + f64x8::splat(4.01122209e-07) * t119 * t1284 + f64x8::splat(6.05687217e-07) * t1282 * t82 - f64x8::splat(1.973847762e-06) * t1282 * t88 - f64x8::splat(0.01216676528) * t1300 - f64x8::splat(0.00608338264) * t44 * t1307 - f64x8::splat(1.00478906e-07) * t1313 * t82 - f64x8::splat(2.00957812e-07) * t1316 - f64x8::splat(1.00478906e-07) * t44 * t1325;
            let t1331 = t1313 * t77;
            let t1333 = t326 * t351;
            let t1335 = t44 * t1278;
            let t1337 = t1282 * t77;
            let t1341 = t44 * t1282;
            let t1343 = t99 * t1313;
            let t1347 = f64x8::splat(15.0) * t1341 + f64x8::splat(15.0) / f64x8::splat(2.0) * t1343 + t1310 / f64x8::splat(48.0) - t1235 / f64x8::splat(96.0);
            let t1352 = t387 * t351;
            let t1354 = t119 * t1278;
            let t1358 = t129 * t1278;
            let t1366 = t1347 * t77;
            let t1372 = f64x8::splat(0.0318024096) * t1313 * t92 + f64x8::splat(0.0453837246) * t1331 + f64x8::splat(0.0907674492) * t1333 + f64x8::splat(0.0453837246) * t1335 - f64x8::splat(0.0667950417) * t1337 + f64x8::splat(3.97324768e-09) * t119 * t1325 + f64x8::splat(1.33707403e-07) * t1347 * t92 - f64x8::splat(6.57949254e-07) * t129 * t1307 + f64x8::splat(0.0384749108) * t1352 + f64x8::splat(0.0192374554) * t1354 + f64x8::splat(2.01895739e-07) * t129 * t1325 - f64x8::splat(0.0222650139) * t1358 - f64x8::splat(0.00608338264) * t1313 * t88 - f64x8::splat(1.099818826e-07) * t387 * t371 - f64x8::splat(5.49909413e-08) * t119 * t1307 + f64x8::splat(0.0192374554) * t1366 + f64x8::splat(3.97324768e-09) * t1347 * t82 + f64x8::splat(7.94649536e-09) * t387 * t362;
            let t1374 = t44 * t1313;
            let t1404 = t408 * t351;
            let t1406 = t106 * t1278;
            let t1408 = t99 * t1282;
            let t1410 = -f64x8::splat(0.0667950417) * t1374 * t77 - f64x8::splat(0.1335900834) * t404 * t351 + f64x8::splat(0.1908144576) * t329 * t351 + f64x8::splat(0.0954072288) * t95 * t1278 - f64x8::splat(3.004496088e-06) * t437 * t351 - f64x8::splat(1.502248044e-06) * t114 * t1278 + f64x8::splat(8.02244418e-07) * t390 * t351 + f64x8::splat(4.01122209e-07) * t126 * t1278 + f64x8::splat(6.05687217e-07) * t1374 * t82 + f64x8::splat(1.211374434e-06) * t404 * t362 - f64x8::splat(1.973847762e-06) * t1374 * t88 - f64x8::splat(3.947695524e-06) * t404 * t371 - f64x8::splat(0.01565454237) * t1374 * t92 - f64x8::splat(0.01565454237) * t136 * t1278 - f64x8::splat(0.0020245016416666666) * t1310 + f64x8::splat(1.838634068e-06) * t1404 + f64x8::splat(9.19317034e-07) * t1406 + f64x8::splat(0.074243981175) * t1408;
            let t1411 = t116 * t1313;
            let t1418 = f64x8::splat(105.0) / f64x8::splat(2.0) * t1408 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1411 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1282 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1374;
            let t1421 = t1418 * t77;
            let t1444 = f64x8::splat(0.024747993725) * t1411 - f64x8::splat(0.170256507825) * t1374 - f64x8::splat(5.00749348e-07) * t1418 * t92 + f64x8::splat(9.19317034e-07) * t1421 + f64x8::splat(1.148635778e-07) * t408 * t371 + f64x8::splat(0.0211191213) * t1302 + f64x8::splat(0.01055956065) * t1304 + f64x8::splat(5.74317889e-08) * t106 * t1307 + f64x8::splat(5.74317889e-08) * t1418 * t88 + f64x8::splat(0.007430350515) * t1318 + f64x8::splat(0.002476783505) * t1320 + f64x8::splat(0.043536371955) * t1323 - f64x8::splat(3.40722258e-09) * t106 * t1325 - f64x8::splat(3.40722258e-09) * t1418 * t82 - f64x8::splat(6.81444516e-09) * t408 * t362 - f64x8::splat(0.301697472) * t1341 - f64x8::splat(0.150848736) * t1343 - f64x8::splat(5.49909413e-08) * t1347 * t88;
            let t1446 = t1328 + t1372 + t1410 + t1444;
            let t1450 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1446));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t1450;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t1467 = t341 * t461;
            let t1473 = t468 * t348;
            let t1478 = t183 * t341;
            let t1481 = f64x8::splat(6.0) * t581 * t1467 + f64x8::splat(24.0) * t1478 * t461 + t1272 + t1274;
            let t1484 = ((t54).select(f64x8::splat(60.0) * t1087 * t333 - f64x8::splat(18.0) * t456 * t333 - f64x8::splat(12.0) * t458 * t333 + t1242 + t1246 - t1250, -f64x8::splat(24.0) * t560 * t1093 * t341 - f64x8::splat(6.0) * t565 * t1107 * t341 - f64x8::splat(6.0) * t565 * t462 * t348 + f64x8::splat(6.0) * t173 * t1467 - f64x8::splat(2.0) * t577 * t1473 + t180 * t1481 + t1263));
            let t1488 = t479 * t351;
            let t1490 = t85 * t1484;
            let t1492 = t351 * t471;
            let t1494 = t77 * t1484;
            let t1496 = f64x8::splat(105.0) / f64x8::splat(2.0) * t1488 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1490 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1492 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1494;
            let t1501 = t474 * t351;
            let t1503 = t78 * t1484;
            let t1506 = f64x8::splat(15.0) * t1501 + f64x8::splat(15.0) / f64x8::splat(2.0) * t1503 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1484;
            let t1515 = t387 * t471;
            let t1518 = t119 * t1484;
            let t1526 = t408 * t471;
            let t1528 = t106 * t1484;
            let t1530 = t44 * t1484;
            let t1532 = t326 * t482;
            let t1537 = t129 * t1484;
            let t1539 = f64x8::splat(0.0192374554) * t1518 + f64x8::splat(2.01895739e-07) * t129 * t1496 - f64x8::splat(5.49909413e-08) * t387 * t482 - f64x8::splat(5.49909413e-08) * t119 * t1506 + f64x8::splat(9.19317034e-07) * t1526 + f64x8::splat(9.19317034e-07) * t1528 + f64x8::splat(0.0453837246) * t1530 - f64x8::splat(0.00608338264) * t1532 + f64x8::splat(0.01055956065) * t1503 - f64x8::splat(0.00608338264) * t44 * t1506 - f64x8::splat(0.0222650139) * t1537;
            let t1541 = t326 * t476;
            let t1548 = t326 * t471;
            let t1558 = -f64x8::splat(1.00478906e-07) * t1541 + f64x8::splat(0.002476783505) * t1490 + f64x8::splat(0.043536371955) * t1492 + f64x8::splat(0.043536371955) * t1494 - f64x8::splat(1.00478906e-07) * t44 * t1496 + f64x8::splat(0.0453837246) * t1548 + f64x8::splat(0.0211191213) * t1501 + f64x8::splat(0.007430350515) * t1488 + f64x8::splat(0.0954072288) * t329 * t471 + f64x8::splat(0.0954072288) * t352 * t471 + f64x8::splat(0.0954072288) * t95 * t1484;
            let t1581 = -f64x8::splat(0.0667950417) * t404 * t471 + f64x8::splat(6.05687217e-07) * t404 * t476 - f64x8::splat(1.973847762e-06) * t404 * t482 - f64x8::splat(0.01565454237) * t374 * t471 - f64x8::splat(0.01565454237) * t136 * t1484 + f64x8::splat(4.01122209e-07) * t390 * t471 + f64x8::splat(4.01122209e-07) * t376 * t471 + f64x8::splat(4.01122209e-07) * t126 * t1484 - f64x8::splat(1.502248044e-06) * t437 * t471 - f64x8::splat(1.502248044e-06) * t439 * t471 - f64x8::splat(1.502248044e-06) * t114 * t1484;
            let t1583 = -f64x8::splat(0.04696362711) * t404 * t474 - f64x8::splat(0.06972770593) * t1484 + f64x8::splat(3.97324768e-09) * t387 * t476 + f64x8::splat(3.97324768e-09) * t119 * t1496 + f64x8::splat(5.74317889e-08) * t408 * t482 + f64x8::splat(5.74317889e-08) * t106 * t1506 - f64x8::splat(3.40722258e-09) * t408 * t476 - f64x8::splat(3.40722258e-09) * t106 * t1496 - f64x8::splat(6.57949254e-07) * t129 * t1506 + f64x8::splat(0.0192374554) * t1515 + t1539 + t1558 + t1581;
            let t1587 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1583));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t1587;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t1589 = t453 * t453;
            let t1592 = t169 * t1589;
            let t1594 = t554 * t1589;
            let t1597 = t461 * t461;
            let t1598 = t67 * t1597;
            let t1606 = t468 * t468;
            let t1613 = f64x8::splat(24.0) * t183 * t1597 + f64x8::splat(6.0) * t581 * t1597 + t1272 + t1274;
            let t1616 = ((t54).select(-f64x8::splat(18.0) * t63 * t1589 + t1242 + t1246 - t1250 - f64x8::splat(12.0) * t1592 + f64x8::splat(60.0) * t1594, -f64x8::splat(12.0) * t565 * t462 * t468 + f64x8::splat(6.0) * t173 * t1597 - f64x8::splat(24.0) * t560 * t1598 - f64x8::splat(2.0) * t577 * t1606 + t180 * t1613 + t1263));
            let t1618 = t471 * t471;
            let t1620 = t77 * t1618;
            let t1622 = t78 * t1616;
            let t1625 = f64x8::splat(15.0) * t1620 + f64x8::splat(15.0) / f64x8::splat(2.0) * t1622 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1616;
            let t1630 = t78 * t1618;
            let t1632 = t85 * t1616;
            let t1635 = t77 * t1616;
            let t1637 = f64x8::splat(105.0) / f64x8::splat(2.0) * t1630 + f64x8::splat(35.0) / f64x8::splat(2.0) * t1632 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1618 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1635;
            let t1656 = -f64x8::splat(0.06972770593) * t1616 + f64x8::splat(0.043536371955) * t1618 - f64x8::splat(6.57949254e-07) * t129 * t1625 + f64x8::splat(0.0192374554) * t119 * t1616 + f64x8::splat(2.01895739e-07) * t129 * t1637 - f64x8::splat(1.502248044e-06) * t106 * t1618 - f64x8::splat(5.49909413e-08) * t119 * t1625 + f64x8::splat(9.19317034e-07) * t106 * t1616 + f64x8::splat(3.97324768e-09) * t119 * t1637 + f64x8::splat(5.74317889e-08) * t106 * t1625 - f64x8::splat(3.40722258e-09) * t106 * t1637 - f64x8::splat(0.01565454237) * t129 * t1618 + f64x8::splat(4.01122209e-07) * t119 * t1618;
            let t1680 = f64x8::splat(0.007430350515) * t1630 + f64x8::splat(0.0453837246) * t44 * t1616 + f64x8::splat(0.0211191213) * t1620 + f64x8::splat(0.01055956065) * t1622 - f64x8::splat(0.00608338264) * t44 * t1625 + f64x8::splat(0.002476783505) * t1632 + f64x8::splat(0.043536371955) * t1635 - f64x8::splat(1.00478906e-07) * t44 * t1637 - f64x8::splat(0.0222650139) * t129 * t1616 + f64x8::splat(0.0954072288) * t44 * t1618 + f64x8::splat(0.0954072288) * t95 * t1616 - f64x8::splat(0.01565454237) * t136 * t1616 + f64x8::splat(4.01122209e-07) * t126 * t1616 - f64x8::splat(1.502248044e-06) * t114 * t1616;
            let t1681 = t1656 + t1680;
            let t1685 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1681));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t1685;
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
