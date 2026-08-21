//! LDA_C_WIGNER kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_wigner_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t12 = param_b + t4 * t6 * t8 / 4.0;
        let tzk0 = param_a / t12;
        zk[ip] += tzk0;
        let t15 = t12 * t12;
        let t16 = 1.0 / t15;
        let tvrho0 = tzk0 + t8 * param_a * t16 * t4 * t6 / 12.0;
        vrho[ip] += tvrho0;
        let t22 = param_a * t16 * t1;
        let t23 = t3 * t6;
        let t28 = t7 * t7;
        let t33 = 1.0 / t15 / t12;
        let t35 = t1 * t1;
        let t36 = t3 * t3;
        let tv2rho20 = t22 * t23 / t7 / rho[ip] / 18.0 + 1.0 / t28 / rho[ip] * param_a * t33 * t35 * t36 * t5 / 18.0;
        v2rho2[ip] += tv2rho20;
        let t42 = param_a * t33 * t35;
        let t43 = t36 * t5;
        let t44 = rho[ip] * rho[ip];
        let t55 = t44 * rho[ip];
        let t58 = t15 * t15;
        let t59 = 1.0 / t58;
        let tv3rho30 = -t42 * t43 / t28 / t44 / 18.0 - 2.0 / 27.0 * t22 * t23 / t7 / t44 + 1.0 / t55 * param_a * t59 * t2 / 6.0;
        v3rho3[ip] += tv3rho30;
    }
}
