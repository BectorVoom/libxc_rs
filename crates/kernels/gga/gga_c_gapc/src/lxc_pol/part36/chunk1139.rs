//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1139/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1139<F: Float>(t10293: F, t28192: F, t33399: F, t9894: F, t1744: F, t8709: F, t15516: F, t2660: F, t11597: F, t9574: F, t9578: F, t11917: F, t3363: F, t9846: F) -> (F, F, F, F, F) {
    let t34088 = t9894 * t33399 * t10293 * t28192;
    let t34090 = t1744 * t8709;
    let t34092 = t2660 * t34090 * t15516;
    let t34095 = t9574 * t11597 * t9578;
    let t34098 = t3363 * t11917 * t9846;
    (t34088, t34090, t34092, t34095, t34098)
}
