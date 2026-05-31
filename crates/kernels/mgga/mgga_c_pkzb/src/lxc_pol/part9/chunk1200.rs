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
    let t20845 = F::cast_from(0.37083333333333333334e-1_f64) * t20716;
    let t20849 = (t17349 - F::cast_from(0.86527777777777777777e-1_f64) * t17351 + F::cast_from(0.37083333333333333333e-1_f64) * t17354 - F::cast_from(0.92708333333333333333e-2_f64) * t17357 - F::cast_from(0.28842592592592592592e-1_f64) * t20705 + t20845 - F::cast_from(0.278125e-1_f64) * t20719 + F::cast_from(0.278125e-1_f64) * t20745) * t261;
    let t20861 = F::cast_from(0.11958666666666666667e1_f64) * t20716;
    let t20868 = -F::cast_from(0.21908444444444444445e1_f64) * t17405 + F::cast_from(0.82156666666666666666e0_f64) * t17411 - F::cast_from(0.49293999999999999999e0_f64) * t17414 - F::cast_from(0.16431333333333333333e0_f64) * t17417 - F::cast_from(0.93011851851851851854e0_f64) * t20705 + F::cast_from(0.427258125e1_f64) * t20708 - F::cast_from(0.230371875e0_f64) * t20710 + t17566 - F::cast_from(0.27903555555555555556e1_f64) * t17351 + F::cast_from(0.11958666666666666667e1_f64) * t17354 - F::cast_from(0.29896666666666666667e0_f64) * t17357 + t20861 - F::cast_from(0.89690000000000000001e0_f64) * t20719 + F::cast_from(0.8969e0_f64) * t20745 + F::cast_from(0.82156666666666666665e0_f64) * t20748 + F::cast_from(0.82156666666666666665e0_f64) * t20751 - F::cast_from(0.73028148148148148147e0_f64) * t20754 + t17548 + F::cast_from(0.82156666666666666666e0_f64) * t17408;
    (t20837, t20849, t20868)
}
