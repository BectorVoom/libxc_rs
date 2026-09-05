//! MGGA_X_MBEEFVDW vxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_mbeefvdw_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
