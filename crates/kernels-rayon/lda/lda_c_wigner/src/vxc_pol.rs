//! LDA_C_WIGNER vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

/// LDA_C_WIGNER vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_wigner_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
