//! MGGA_X_GX exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gx_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c0 = f64x8::splat(param_c0);
    let param_c1 = f64x8::splat(param_c1);
    let param_alphainf = f64x8::splat(param_alphainf);
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
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_CBRT4);
            let t26 = f64x8::splat(8.0) / f64x8::splat(27.0) * t21 * t22 * t24;
            let t27 = t21 * t21;
            let t28 = v_tau * t27;
            let t29 = t19 * t19;
            let t31 = f64x8::splat(1.0) / t29 / v_rho;
            let t33 = v_sigma * t27;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t29 / t34;
            let t39 = t28 * t31 - t33 * t36 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t39 * t40 * t45;
            let t48 = t40 * t45;
            let t51 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t39 * t48;
            let t52 = param_c0 + param_c1 - f64x8::splat(1.0);
            let t56 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t39 * t48;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = f64x8::splat(1.0) - t26;
            let t60 = t51 * t57 * t59;
            let t63 = t26 + f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t60;
            let t64 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46;
            let t65 = f64x8::splat(1.0) - t64;
            let t66 = ((t65).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t68 = f64x8::splat(1.0) - param_alphainf;
            let t69 = t68 * t65;
            let t70 = f64x8::splat(1.0) + t64;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t69 * t71 + f64x8::splat(1.0);
            let t74 = -t65;
            let t75 = ((t74).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t77 = t63 * t66 + t73 * t75;
            let t81 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t77));
            let tzk0 = f64x8::splat(2.0) * t81;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
