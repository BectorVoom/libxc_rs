//! GGA_X_HCTH_A fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hcth_a.c`
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
pub fn gga_x_hcth_a_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
            let t20 = t3 * t3;
            let t22 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t20 / t22 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = ((v_sigma).sqrt());
            let t35 = t34 * t27;
            let t37 = f64x8::splat(1.0) / t18 / v_rho;
            let t39 = (simd::ln(t35 * t37 + ((((t35 * t37) * (t35 * t37)) + f64x8::splat(1.0)).sqrt())));
            let t40 = t37 * t39;
            let t43 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t35 * t40;
            let t46 = t43 * t43;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = -f64x8::splat(2.51173) / t43 + f64x8::splat(3.7198333333333333) * t47;
            let t54 = f64x8::splat(1.09878) + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t49;
            let t58 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t54));
            let tzk0 = f64x8::splat(2.0) * t58;
            acc_zk = tzk0;
            let t60 = t17 / t31;
            let t64 = t30 * v_rho;
            let t66 = f64x8::splat(1.0) / t31 / t64;
            let t73 = f64x8::splat(1.0) / t18 / t30 * t39;
            let t77 = t29 * t33 + f64x8::splat(1.0);
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t66 * t79;
            let t83 = -f64x8::splat(0.0336) * t35 * t73 - f64x8::splat(0.0336) * t29 * t80;
            let t87 = f64x8::splat(1.0) / t46 / t43;
            let t88 = t87 * t83;
            let t90 = f64x8::splat(2.51173) * t47 * t83 - f64x8::splat(7.439666666666667) * t88;
            let t95 = -f64x8::splat(0.002488888888888889) * t26 * t29 * t66 * t49 + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t90;
            let t100 = ((t2).select(f64x8::splat(0.0), -t6 * t60 * t54 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t95));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t100 + f64x8::splat(2.0) * t58;
            acc_vrho = tvrho0;
            let t103 = t28 * t33;
            let t108 = f64x8::splat(1.0) / t34 * t27;
            let t113 = f64x8::splat(0.0126) * t108 * t40 + f64x8::splat(0.0126) * t103 * t79;
            let t116 = t87 * t113;
            let t118 = f64x8::splat(2.51173) * t47 * t113 - f64x8::splat(7.439666666666667) * t116;
            let t123 = f64x8::splat(0.0009333333333333333) * t26 * t103 * t49 + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t118;
            let t127 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t123));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t127;
            acc_vsigma = tvsigma0;
            let t132 = t17 / t31 / v_rho;
            let t139 = t30 * t30;
            let t141 = f64x8::splat(1.0) / t31 / t139;
            let t150 = t83 * t83;
            let t155 = f64x8::splat(1.0) / t18 / t64 * t39;
            let t158 = t141 * t79;
            let t161 = v_sigma * v_sigma;
            let t162 = t161 * t27;
            let t165 = f64x8::splat(1.0) / t18 / t139 / t64;
            let t167 = f64x8::splat(1.0) / t78 / t77;
            let t171 = f64x8::splat(0.0784) * t35 * t155 + f64x8::splat(0.168) * t29 * t158 - f64x8::splat(0.0896) * t162 * t165 * t167;
            let t174 = t46 * t46;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t175 * t150;
            let t180 = -f64x8::splat(5.02346) * t87 * t150 + f64x8::splat(2.51173) * t47 * t171 + f64x8::splat(22.319) * t176 - f64x8::splat(7.439666666666667) * t87 * t171;
            let t185 = f64x8::splat(0.009125925925925926) * t26 * t29 * t141 * t49 - f64x8::splat(0.004977777777777778) * t26 * t29 * t66 * t90 + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t180;
            let t190 = ((t2).select(f64x8::splat(0.0), t6 * t132 * t54 / f64x8::splat(12.0) - t6 * t60 * t95 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t185));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t190 + f64x8::splat(4.0) * t100;
            acc_v2rho2 = tv2rho20;
            let t196 = t28 * t66;
            let t213 = t139 * t30;
            let t215 = f64x8::splat(1.0) / t18 / t213;
            let t216 = t27 * t215;
            let t217 = t167 * v_sigma;
            let t220 = -f64x8::splat(0.0168) * t108 * t73 - f64x8::splat(0.0504) * t196 * t79 + f64x8::splat(0.0336) * t216 * t217;
            let t223 = t175 * t113;
            let t226 = t87 * t220;
            let t228 = -f64x8::splat(5.02346) * t116 * t83 + f64x8::splat(2.51173) * t47 * t220 + f64x8::splat(22.319) * t223 * t83 - f64x8::splat(7.439666666666667) * t226;
            let t233 = -f64x8::splat(0.002488888888888889) * t26 * t196 * t49 + f64x8::splat(0.0009333333333333333) * t26 * t103 * t90 - f64x8::splat(0.002488888888888889) * t26 * t29 * t66 * t118 + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t228;
            let t238 = ((t2).select(f64x8::splat(0.0), -t6 * t60 * t123 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t233));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t238 + f64x8::splat(2.0) * t127;
            acc_v2rhosigma = tv2rhosigma0;
            let t244 = t113 * t113;
            let t249 = f64x8::splat(1.0) / t34 / v_sigma * t27;
            let t252 = f64x8::splat(1.0) / v_sigma;
            let t253 = t252 * t28;
            let t254 = t33 * t79;
            let t257 = t139 * v_rho;
            let t259 = f64x8::splat(1.0) / t18 / t257;
            let t263 = -f64x8::splat(0.0063) * t249 * t40 + f64x8::splat(0.0063) * t253 * t254 - f64x8::splat(0.0126) * t27 * t259 * t167;
            let t266 = t175 * t244;
            let t268 = t87 * t263;
            let t270 = -f64x8::splat(5.02346) * t87 * t244 + f64x8::splat(2.51173) * t47 * t263 + f64x8::splat(22.319) * t266 - f64x8::splat(7.439666666666667) * t268;
            let t275 = f64x8::splat(0.0018666666666666666) * t26 * t103 * t118 + f64x8::splat(0.0009333333333333333) * t26 * t29 * t33 * t270;
            let t279 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t275));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t279;
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
