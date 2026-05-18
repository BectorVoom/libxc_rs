//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1200/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1200<F: Float>(t1107: F, t5845: F, t20716: F, t17349: F, t17351: F, t17354: F, t17357: F, t20705: F, t20719: F, t20745: F, t261: F, t17405: F, t17408: F, t17411: F, t17414: F, t17417: F, t17548: F, t17566: F, t20708: F, t20710: F, t20748: F, t20751: F, t20754: F) -> (F, F, F) {
    let t20837 = t5845 * t1107;
    let t20845 = F::new(0.37083333333333333334e-1) * t20716;
    let t20849 = (t17349 - F::new(0.86527777777777777777e-1) * t17351 + F::new(0.37083333333333333333e-1) * t17354 - F::new(0.92708333333333333333e-2) * t17357 - F::new(0.28842592592592592592e-1) * t20705 + t20845 - F::new(0.278125e-1) * t20719 + F::new(0.278125e-1) * t20745) * t261;
    let t20861 = F::new(0.11958666666666666667e1) * t20716;
    let t20868 = -F::new(0.21908444444444444445e1) * t17405 + F::new(0.82156666666666666666e0) * t17411 - F::new(0.49293999999999999999e0) * t17414 - F::new(0.16431333333333333333e0) * t17417 - F::new(0.93011851851851851854e0) * t20705 + F::new(0.427258125e1) * t20708 - F::new(0.230371875e0) * t20710 + t17566 - F::new(0.27903555555555555556e1) * t17351 + F::new(0.11958666666666666667e1) * t17354 - F::new(0.29896666666666666667e0) * t17357 + t20861 - F::new(0.89690000000000000001e0) * t20719 + F::new(0.8969e0) * t20745 + F::new(0.82156666666666666665e0) * t20748 + F::new(0.82156666666666666665e0) * t20751 - F::new(0.73028148148148148147e0) * t20754 + t17548 + F::new(0.82156666666666666666e0) * t17408;
    (t20837, t20849, t20868)
}
