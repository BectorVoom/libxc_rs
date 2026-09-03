//! GGA_XC_TH3 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th3.c`
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
pub fn gga_xc_th3_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * t2;
            let t4 = t3 * t3;
            let t6 = param_omega_0 * t4 * t2;
            let t7 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t8 = t7 * v_rho;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = t12 * t12;
            let t14 = param_omega_1 * t13;
            let t15 = (simd::cbrt(v_rho));
            let t16 = t15 * v_rho;
            let t20 = f64x8::splat(M_SQRT2);
            let t21 = param_omega_2 * t20;
            let t22 = ((v_rho).sqrt());
            let t23 = t22 * v_rho;
            let t27 = param_omega_3 * t12;
            let t28 = t15 * t15;
            let t29 = t28 * v_rho;
            let t33 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t34 = t33 * t33;
            let t36 = t34 * t34;
            let t38 = param_omega_4 * t36 * t34 * t33;
            let t39 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t40 = ((v_sigma).sqrt());
            let t43 = (simd::cbrt(zeta_threshold));
            let t45 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t43 * zeta_threshold, f64x8::splat(1.0)));
            let t50 = param_omega_5 * t20;
            let t56 = param_omega_6 * t12;
            let t62 = param_omega_7 * t2;
            let t68 = param_omega_8 * t12;
            let t69 = f64x8::splat(1.0) / v_rho;
            let t71 = t45 * t45;
            let t76 = param_omega_9 * t2;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = t78 * t7;
            let t80 = f64x8::splat(1.0) / t79;
            let t85 = param_omega_10;
            let t86 = f64x8::splat(1.0) / t28;
            let t87 = t85 * t86;
            let t88 = v_sigma * t71;
            let t92 = param_omega_11 * t12;
            let t93 = v_rho * v_rho;
            let t95 = f64x8::splat(1.0) / t28 / t93;
            let t96 = v_sigma * t95;
            let t98 = t96 * t71 - t96;
            let t103 = param_omega_12 * t2;
            let t104 = t79 * v_rho;
            let t108 = param_omega_13;
            let t109 = t108 * t93;
            let t112 = param_omega_18;
            let t113 = (simd::pow(v_rho, f64x8::splat(1.0833333333333333)));
            let t116 = t6 * t8 / f64x8::splat(2.0) + t14 * t16 / f64x8::splat(2.0) + t21 * t23 / f64x8::splat(2.0) + t27 * t29 / f64x8::splat(2.0) + t38 * t39 * t40 * t45 / f64x8::splat(4.0) + t50 * t7 * t40 * t45 / f64x8::splat(4.0) + t56 * t15 * t40 * t45 / f64x8::splat(4.0) + t62 * t22 * t40 * t45 / f64x8::splat(4.0) + t68 * t69 * v_sigma * t71 / f64x8::splat(8.0) + t76 * t80 * v_sigma * t71 / f64x8::splat(8.0) + t87 * t88 / f64x8::splat(8.0) + t92 * t29 * t98 / f64x8::splat(2.0) + t103 * t104 * t98 / f64x8::splat(2.0) + t109 * t98 / f64x8::splat(2.0) + f64x8::splat(0.9438743126816935) * t112 * t113;
            let tzk0 = t116 * t69;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
