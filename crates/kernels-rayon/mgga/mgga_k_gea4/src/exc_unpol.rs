//! MGGA_K_GEA4 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
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
pub fn mgga_k_gea4_exc_unpol(
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
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t30 = t25 / t28;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t40 = v_lapl * t32;
            let t42 = f64x8::splat(1.0) / t23 / v_rho;
            let t46 = t25 * t25;
            let t48 = f64x8::splat(1.0) / t27 / t26;
            let t49 = t46 * t48;
            let t50 = v_lapl * v_lapl;
            let t51 = t50 * t31;
            let t52 = t34 * v_rho;
            let t54 = f64x8::splat(1.0) / t22 / t52;
            let t58 = t49 * v_sigma;
            let t59 = t34 * t34;
            let t61 = f64x8::splat(1.0) / t22 / t59;
            let t63 = t31 * t61 * v_lapl;
            let t66 = v_sigma * v_sigma;
            let t67 = t66 * t31;
            let t68 = t59 * v_rho;
            let t70 = f64x8::splat(1.0) / t22 / t68;
            let t74 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t30 * t33 * t36 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t40 * t42 + t49 * t51 * t54 / f64x8::splat(2916.0) - t58 * t63 / f64x8::splat(2592.0) + t49 * t67 * t70 / f64x8::splat(8748.0);
            let t78 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
