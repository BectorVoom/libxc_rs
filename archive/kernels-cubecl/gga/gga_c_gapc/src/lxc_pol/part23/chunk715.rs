//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 715/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk715<F: Float>(t5: F, t8448: F, t116: F, t4048: F, t134: F, t667: F, t5589: F, t674: F, t2945: F, t8316: F, t2902: F, t2910: F) -> (F, F, F, F, F, F, F) {
    let t8449 = t5 * t8448;
    let t8450 = t116 * t4048;
    let t8451 = t8449 * t8450;
    let t8452 = t667 * t134;
    let t8454 = t8452 * t674 * t5589;
    let t8455 = t8451 * t8454;
    let t8457 = t8316 * t2945;
    let t8459 = t2902 * t2910;
    (t8449, t8450, t8451, t8452, t8455, t8457, t8459)
}
