//! MGGA_C_CS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`
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
pub fn mgga_c_cs_exc_unpol(
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
            let t2 = (simd::cbrt(v_rho));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = f64x8::splat(1.0) + f64x8::splat(0.349) * t3;
            let t6 = f64x8::splat(1.0) / t5;
            let t8 = (simd::exp(-f64x8::splat(0.2533) * t3));
            let t10 = zeta_threshold * zeta_threshold;
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = t11 * t11;
            let t14 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t12 * t10, f64x8::splat(1.0)));
            let t15 = f64x8::splat(M_CBRT2);
            let t16 = t14 * t15;
            let t17 = t15 * t15;
            let t18 = v_tau * t17;
            let t19 = t2 * t2;
            let t21 = f64x8::splat(1.0) / t19 / v_rho;
            let t23 = v_lapl * t17;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t19 / t29;
            let t36 = t16 * (t18 * t21 - t23 * t21 / f64x8::splat(8.0)) / f64x8::splat(4.0) - v_sigma * t31 / f64x8::splat(8.0) + v_lapl * t21 / f64x8::splat(8.0);
            let t39 = f64x8::splat(1.0) + f64x8::splat(0.264) * t8 * t36;
            let tzk0 = -f64x8::splat(0.04918) * t6 * t39;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
