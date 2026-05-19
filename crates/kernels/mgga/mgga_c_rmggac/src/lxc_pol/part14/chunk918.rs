//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 918/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk918<F: Float>(t34884: F, t9046: F, t2289: F, t34881: F, t16501: F, t7363: F, t1966: F, t34976: F, t352: F, t38422: F, t4550: F, t1180: F, t34759: F) -> (F, F, F, F, F, F) {
    let t39840 = t34884 * t9046;
    let t39841 = F::cast_from(0.24829349937757072982e-4_f64) * t39840;
    let t39842 = t34881 * t2289;
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    let t39855 = t39851 * t34976 * t38422 * t4550 * t352;
    let t39857 = t1180 * t34759;
    (t39841, t39842, t39850, t39851, t39855, t39857)
}
