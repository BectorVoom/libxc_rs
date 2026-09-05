//! HYB_MGGA_X_DLDF exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
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
pub fn hyb_mgga_x_dldf_exc_unpol(
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
            let t5 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t6 = zeta_threshold - f64x8::splat(1.0);
            let t8 = ((t5).select(t6, (t5).select(-t6, f64x8::splat(0.0))));
            let t9 = f64x8::splat(1.0) + t8;
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = (simd::cbrt(t9));
            let t15 = (((t9).simd_le(zeta_threshold)).select(t11 * zeta_threshold, t13 * t9));
            let t16 = t4 * t15;
            let t17 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(M_CBRT6);
            let t19 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = t20 * t20;
            let t23 = t18 / t21;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = v_sigma * t25;
            let t27 = v_rho * v_rho;
            let t28 = t17 * t17;
            let t30 = f64x8::splat(1.0) / t28 / t27;
            let t34 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t23 * t26 * t30;
            let t37 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t34;
            let t38 = t17 * t37;
            let t39 = t18 * t18;
            let t41 = f64x8::splat(3.0) / f64x8::splat(10.0) * t39 * t21;
            let t42 = v_tau * t25;
            let t44 = f64x8::splat(1.0) / t28 / v_rho;
            let t45 = t42 * t44;
            let t46 = t41 - t45;
            let t47 = t41 + t45;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = t46 * t46;
            let t52 = t47 * t47;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = t51 * t46;
            let t57 = t52 * t47;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t51 * t51;
            let t62 = t52 * t52;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t46 * t48 - f64x8::splat(0.1880028) * t51 * t53 - f64x8::splat(0.4490609) * t56 * t58 - f64x8::splat(0.0082359) * t61 * t63;
            let t70 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t16 * t38 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
