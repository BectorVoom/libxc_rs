//! GGA_X_OL2 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`
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
pub fn gga_x_ol2_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_bb = f64x8::splat(param_bb);
    let param_cc = f64x8::splat(param_cc);
    let param_aa = f64x8::splat(param_aa);
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
            let t20 = param_bb * v_sigma;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t27 = t22 * t26;
            let t30 = ((v_sigma).sqrt());
            let t31 = param_cc * t30;
            let t33 = f64x8::splat(1.0) / t18 / v_rho;
            let t38 = f64x8::splat(4.0) * t30 * t21 * t33 + t21;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t21 * t33 * t39;
            let t42 = param_aa + f64x8::splat(0.013888888888888888) * t20 * t27 + t31 * t40;
            let t46 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t42));
            let tzk0 = f64x8::splat(2.0) * t46;
            acc_zk = tzk0;
            let t48 = t17 / t24;
            let t52 = t23 * v_rho;
            let t54 = f64x8::splat(1.0) / t24 / t52;
            let t55 = t22 * t54;
            let t61 = t21 / t18 / t23 * t39;
            let t64 = param_cc * v_sigma;
            let t65 = t38 * t38;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t55 * t66;
            let t70 = -f64x8::splat(0.037037037037037035) * t20 * t55 - f64x8::splat(4.0) / f64x8::splat(3.0) * t31 * t61 + f64x8::splat(16.0) / f64x8::splat(3.0) * t64 * t67;
            let t75 = ((t2).select(f64x8::splat(0.0), -t6 * t48 * t42 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t70));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t75 + f64x8::splat(2.0) * t46;
            acc_vrho = tvrho0;
            let t78 = param_bb * t22;
            let t81 = f64x8::splat(1.0) / t30;
            let t82 = param_cc * t81;
            let t85 = param_cc * t22;
            let t89 = f64x8::splat(0.013888888888888888) * t78 * t26 + t82 * t40 / f64x8::splat(2.0) - f64x8::splat(2.0) * t85 * t26 * t66;
            let t93 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t89));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t93;
            acc_vsigma = tvsigma0;
            let t98 = t17 / t24 / v_rho;
            let t105 = t23 * t23;
            let t107 = f64x8::splat(1.0) / t24 / t105;
            let t108 = t22 * t107;
            let t114 = t21 / t18 / t52 * t39;
            let t117 = t108 * t66;
            let t120 = t30 * v_sigma;
            let t121 = param_cc * t120;
            let t122 = t105 * t23;
            let t123 = f64x8::splat(1.0) / t122;
            let t125 = f64x8::splat(1.0) / t65 / t38;
            let t126 = t123 * t125;
            let t129 = f64x8::splat(0.13580246913580246) * t20 * t108 + f64x8::splat(28.0) / f64x8::splat(9.0) * t31 * t114 - f64x8::splat(80.0) / f64x8::splat(3.0) * t64 * t117 + f64x8::splat(1024.0) / f64x8::splat(9.0) * t121 * t126;
            let t134 = ((t2).select(f64x8::splat(0.0), t6 * t98 * t42 / f64x8::splat(12.0) - t6 * t48 * t70 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t129));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t134 + f64x8::splat(4.0) * t75;
            acc_v2rho2 = tv2rho20;
            let t147 = t105 * v_rho;
            let t148 = f64x8::splat(1.0) / t147;
            let t150 = t125 * t30;
            let t153 = -f64x8::splat(0.037037037037037035) * t78 * t54 - f64x8::splat(2.0) / f64x8::splat(3.0) * t82 * t61 + f64x8::splat(8.0) * t85 * t54 * t66 - f64x8::splat(128.0) / f64x8::splat(3.0) * param_cc * t148 * t150;
            let t158 = ((t2).select(f64x8::splat(0.0), -t6 * t48 * t89 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t153));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t158 + f64x8::splat(2.0) * t93;
            acc_v2rhosigma = tv2rhosigma0;
            let t161 = f64x8::splat(1.0) / t120;
            let t162 = param_cc * t161;
            let t165 = f64x8::splat(1.0) / v_sigma;
            let t166 = param_cc * t165;
            let t167 = t27 * t66;
            let t169 = f64x8::splat(1.0) / t105;
            let t174 = -t162 * t40 / f64x8::splat(4.0) - t166 * t167 + f64x8::splat(16.0) * param_cc * t169 * t125 * t81;
            let t178 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t174));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t178;
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
