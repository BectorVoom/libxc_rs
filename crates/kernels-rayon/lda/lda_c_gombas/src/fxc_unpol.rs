//! LDA_C_GOMBAS fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gombas_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.0562 * t2;
        let t6 = 0.0357 / t4;
        let t7 = t2 + 2.39;
        let t9 = f64::ln(t7 * t1);
        let t10 = 0.0311 * t9;
        let tzk0 = -t6 - t10;
        zk[ip] += tzk0;
        let t11 = t4 * t4;
        let t12 = 1.0 / t11;
        let t14 = 1.0 / t1 / rho[ip];
        let t15 = t12 * t14;
        let t18 = t1 * t1;
        let t22 = -1.0 / rho[ip] / 3.0 + t7 / t18 / 3.0;
        let t23 = 1.0 / t7;
        let t24 = t22 * t23;
        let t25 = t24 * t2;
        let tvrho0 = -t6 - t10 + rho[ip] * (-0.00066878 * t15 - 0.0311 * t25);
        vrho[ip] += tvrho0;
        let t32 = 1.0 / t11 / t4;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t18 / t33;
        let t36 = t32 * t35;
        let t39 = 1.0 / t1 / t33;
        let t40 = t12 * t39;
        let t44 = 1.0 / t18 / rho[ip];
        let t47 = 2.0 / 9.0 / t33 - 2.0 / 9.0 * t7 * t44;
        let t48 = t47 * t23;
        let t49 = t48 * t2;
        let t51 = t7 * t7;
        let t52 = 1.0 / t51;
        let t53 = t22 * t52;
        let t54 = t53 * t44;
        let t56 = t24 * t14;
        let tv2rho20 = -0.00133756 * t15 - 0.0622 * t25 + rho[ip] * (-2.5056957333333333e-05 * t36 + 0.0008917066666666667 * t40 - 0.0311 * t49 - 0.010366666666666666 * t54 + 0.010366666666666666 * t56);
        v2rho2[ip] += tv2rho20;
    }
}
