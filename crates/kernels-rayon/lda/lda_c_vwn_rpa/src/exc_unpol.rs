//! LDA_C_VWN_RPA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_rpa.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_rpa_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
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
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = rmath::sqrt(t10);
        let t14 = t11 + 6.536 * t12 + 42.7198;
        let t15 = 1.0 / t14;
        let t19 = rmath::ln(t4 * t9 * t15 / 4.0);
        let t21 = t12 + 13.072;
        let t24 = rmath::atan(0.0448998886412873 / t21);
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.409286;
        let t28 = t27 * t27;
        let t30 = rmath::ln(t28 * t15);
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(1.0 <= zeta_threshold, t34 * zeta_threshold, 1.0);
        let t38 = 2.0 * t36 - 2.0;
        let t39 = M_CBRT2;
        let t42 = 1.0 / (2.0 * t39 - 2.0);
        let t44 = -t38 * t42 + 1.0;
        let t45 = (0.0310907 * t19 + 20.521972937837504 * t24 + 0.004431373767749538 * t30) * t44;
        let t47 = t11 + 10.06155 * t12 + 101.578;
        let t48 = 1.0 / t47;
        let t52 = rmath::ln(t4 * t9 * t48 / 4.0);
        let t54 = t12 + 20.1231;
        let t57 = rmath::atan(1.171685277708993 / t54);
        let t59 = t26 + 0.743294;
        let t60 = t59 * t59;
        let t62 = rmath::ln(t60 * t48);
        let t66 = (0.01554535 * t52 + 0.6188180297906063 * t57 + 0.002667310007273315 * t62) * t38 * t42;
        let tzk0 = t45 + t66;
        zk[ip] += tzk0;
    }
}
