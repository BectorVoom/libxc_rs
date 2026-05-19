//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 200/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk200<F: Float>(t231: F, t242: F, t344: F, t366: F, t4: F, t55: F, t706: F, t713: F, t719: F, t720: F, t79: F, t1: F) -> (F, F) {
    let t724 = t231 * (F::cast_from(0.53236443333333333332e-3_f64) * t4 * t79 * t242 + F::new(1.0) * t706 * t713 - t344 - t366 + F::cast_from(0.18311555036753159941e-3_f64) * t4 * t79 * t55 + F::cast_from(0.58482233974552040708e0_f64) * t719 * t720);
    let t725 = t231 * t1;
    (t724, t725)
}
