//! LDA_XC_TETER93 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_teter93.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_teter93_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * zeta_threshold;
        let t9 = pow_1_3(t5);
        let t11 = piecewise3(t6, t8, t9 * t5);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(t12);
        let t16 = piecewise3(t13, t8, t14 * t12);
        let t18 = M_CBRT2;
        let t21 = 1.0 / (2.0 * t18 - 2.0);
        let t22 = (t11 + t16 - 2.0) * t21;
        let t26 = M_CBRT3;
        let t27 = (2.217058676663745 + 0.6157402568883344 * t22) * t26;
        let t28 = 1.0 / M_PI;
        let t29 = pow_1_3(t28);
        let t30 = M_CBRT4;
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = pow_1_3(t2);
        let t34 = 1.0 / t33;
        let t35 = t32 * t34;
        let t40 = t26 * t26;
        let t41 = (0.7405551735357053 + 0.1574201515892867 * t22) * t40;
        let t42 = t29 * t29;
        let t43 = t42 * t30;
        let t44 = t33 * t33;
        let t46 = t43 / t44;
        let t51 = (0.01968227878617998 + 0.003532336663397157 * t22) * t28;
        let t54 = 0.4581652932831429 + 0.119086804055547 * t22 + t27 * t35 / 4.0 + t41 * t46 / 4.0 + 3.0 / 4.0 * t51 * t3;
        let t55 = t26 * t29;
        let t61 = (4.504130959426697 + 0.2673612973836267 * t22) * t40;
        let t66 = (1.110667363742916 + 0.2052004607777787 * t22) * t28;
        let t71 = (0.02359291751427506 + 0.004200005045691381 * t22) * t26;
        let t73 = t29 * t28 * t31;
        let t75 = 1.0 / t33 / t2;
        let t76 = t73 * t75;
        let t79 = 0.25 * t55 * t31 * t34 + t61 * t46 / 4.0 + 3.0 / 4.0 * t66 * t3 + 3.0 / 16.0 * t71 * t76;
        let t80 = 1.0 / t79;
        let tzk0 = -t54 * t80;
        zk[ip] += tzk0;
    }
}
