//! LDA_C_GOMBAS fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gombas_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.0562 * t3;
        let t7 = 0.0357 / t5;
        let t8 = t3 + 2.39;
        let t10 = rmath::ln(t8 * t2);
        let t11 = 0.0311 * t10;
        let tzk0 = -t7 - t11;
        zk[ip] += tzk0;
        let t12 = t5 * t5;
        let t13 = 1.0 / t12;
        let t15 = 1.0 / t2 / t1;
        let t16 = t13 * t15;
        let t19 = t2 * t2;
        let t23 = -1.0 / t1 / 3.0 + t8 / t19 / 3.0;
        let t24 = 1.0 / t8;
        let t25 = t23 * t24;
        let t26 = t25 * t3;
        let tvrho0 = -t7 - t11 + t1 * (-0.00066878 * t16 - 0.0311 * t26);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t33 = 1.0 / t12 / t5;
        let t34 = t1 * t1;
        let t36 = 1.0 / t19 / t34;
        let t37 = t33 * t36;
        let t40 = 1.0 / t2 / t34;
        let t41 = t13 * t40;
        let t45 = 1.0 / t19 / t1;
        let t48 = 2.0 / 9.0 / t34 - 2.0 / 9.0 * t8 * t45;
        let t49 = t48 * t24;
        let t50 = t49 * t3;
        let t52 = t8 * t8;
        let t53 = 1.0 / t52;
        let t54 = t23 * t53;
        let t55 = t54 * t45;
        let t57 = t25 * t15;
        let tv2rho20 = -0.00133756 * t16 - 0.0622 * t26 + t1 * (-2.5056957333333333e-05 * t37 + 0.0008917066666666667 * t41 - 0.0311 * t50 - 0.010366666666666666 * t55 + 0.010366666666666666 * t57);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
