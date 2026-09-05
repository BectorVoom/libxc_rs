//! GGA_X_N12 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
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
pub fn gga_x_n12_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_CC_0_1 = f64x8::splat(param_CC_0_1);
    let param_CC_0_2 = f64x8::splat(param_CC_0_2);
    let param_CC_0_3 = f64x8::splat(param_CC_0_3);
    let param_CC_1_1 = f64x8::splat(param_CC_1_1);
    let param_CC_1_2 = f64x8::splat(param_CC_1_2);
    let param_CC_1_3 = f64x8::splat(param_CC_1_3);
    let param_CC_1_0 = f64x8::splat(param_CC_1_0);
    let param_CC_2_1 = f64x8::splat(param_CC_2_1);
    let param_CC_2_2 = f64x8::splat(param_CC_2_2);
    let param_CC_2_3 = f64x8::splat(param_CC_2_3);
    let param_CC_2_0 = f64x8::splat(param_CC_2_0);
    let param_CC_3_1 = f64x8::splat(param_CC_3_1);
    let param_CC_3_2 = f64x8::splat(param_CC_3_2);
    let param_CC_3_3 = f64x8::splat(param_CC_3_3);
    let param_CC_3_0 = f64x8::splat(param_CC_3_0);
    let param_CC_0_0 = f64x8::splat(param_CC_0_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t12 = (t11).simd_le(zeta_threshold);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = ((t12).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t21 = param_CC_0_1;
            let t22 = t21 * v_sigma;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = t23 * t23;
            let t25 = v_rho * v_rho;
            let t26 = t18 * t18;
            let t28 = f64x8::splat(1.0) / t26 / t25;
            let t29 = t24 * t28;
            let t33 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma * t24 * t28;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t29 * t34;
            let t38 = param_CC_0_2;
            let t39 = v_sigma * v_sigma;
            let t40 = t38 * t39;
            let t41 = t25 * t25;
            let t42 = t41 * v_rho;
            let t44 = f64x8::splat(1.0) / t18 / t42;
            let t46 = t33 * t33;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t23 * t44 * t47;
            let t51 = param_CC_0_3;
            let t52 = t39 * v_sigma;
            let t53 = t51 * t52;
            let t54 = t41 * t41;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t46 * t33;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t55 * t57;
            let t62 = param_CC_1_1;
            let t63 = t62 * v_sigma;
            let t66 = param_CC_1_2;
            let t67 = t66 * t39;
            let t70 = param_CC_1_3;
            let t71 = t70 * t52;
            let t74 = param_CC_1_0 + f64x8::splat(0.004) * t63 * t35 + f64x8::splat(3.2e-05) * t67 * t48 + f64x8::splat(2.56e-07) * t71 * t58;
            let t79 = ((t12).select(f64x8::splat(1.0) / t13, f64x8::splat(1.0) / t15));
            let t82 = f64x8::splat(1.0) + f64x8::splat(0.4) / t18 * t23 * t79;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = param_CC_2_1;
            let t87 = t86 * v_sigma;
            let t90 = param_CC_2_2;
            let t91 = t90 * t39;
            let t94 = param_CC_2_3;
            let t95 = t94 * t52;
            let t98 = param_CC_2_0 + f64x8::splat(0.004) * t87 * t35 + f64x8::splat(3.2e-05) * t91 * t48 + f64x8::splat(2.56e-07) * t95 * t58;
            let t99 = t82 * t82;
            let t100 = f64x8::splat(1.0) / t99;
            let t103 = param_CC_3_1;
            let t104 = t103 * v_sigma;
            let t107 = param_CC_3_2;
            let t108 = t107 * t39;
            let t111 = param_CC_3_3;
            let t112 = t111 * t52;
            let t115 = param_CC_3_0 + f64x8::splat(0.004) * t104 * t35 + f64x8::splat(3.2e-05) * t108 * t48 + f64x8::splat(2.56e-07) * t112 * t58;
            let t116 = t99 * t82;
            let t117 = f64x8::splat(1.0) / t116;
            let t119 = param_CC_0_0 + f64x8::splat(0.004) * t22 * t35 + f64x8::splat(3.2e-05) * t40 * t48 + f64x8::splat(2.56e-07) * t53 * t58 + t74 * t83 + t98 * t100 + t115 * t117;
            let t123 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t119));
            let tzk0 = f64x8::splat(2.0) * t123;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
