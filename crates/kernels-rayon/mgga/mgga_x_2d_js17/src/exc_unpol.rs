//! MGGA_X_2D_JS17 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`
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
pub fn mgga_x_2d_js17_exc_unpol(
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
            let t4 = ((f64x8::splat(M_PI)).sqrt());
            let t6 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t7 = zeta_threshold - f64x8::splat(1.0);
            let t9 = ((t6).select(t7, (t6).select(-t7, f64x8::splat(0.0))));
            let t10 = f64x8::splat(1.0) + t9;
            let t12 = ((zeta_threshold).sqrt());
            let t14 = ((t10).sqrt());
            let t16 = (((t10).simd_le(zeta_threshold)).select(t12 * zeta_threshold, t14 * t10));
            let t17 = f64x8::splat(1.0) / t4 * t16;
            let t18 = f64x8::splat(M_SQRT2);
            let t19 = ((v_rho).sqrt());
            let t20 = t18 * t19;
            let t21 = v_rho * v_rho;
            let t22 = t21 * v_rho;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = v_sigma * t23;
            let t26 = v_sigma * v_sigma;
            let t27 = t21 * t21;
            let t29 = f64x8::splat(1.0) / t27 / t21;
            let t32 = f64x8::splat(1.0) + f64x8::splat(0.8250592249883855) * t24 + f64x8::splat(0.0025211952768090192) * t26 * t29;
            let t33 = (simd::pow(t32, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t43 = f64x8::splat(1.0) + f64x8::splat(0.05587702687752028) * t24 + (-f64x8::splat(0.1544) * v_tau / t21 - f64x8::splat(11.596246802930645)) / f64x8::splat(M_PI) / f64x8::splat(4.0);
            let t44 = (simd::pow(t32, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = f64x8::splat(1.0) / t33 + f64x8::splat(2.0) / f64x8::splat(5.0) * t43 * t45;
            let t52 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t17 * t20 * t48));
            let tzk0 = f64x8::splat(2.0) * t52;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
