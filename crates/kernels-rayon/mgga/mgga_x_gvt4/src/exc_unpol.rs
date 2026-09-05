//! MGGA_X_GVT4 exc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_gvt4_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
