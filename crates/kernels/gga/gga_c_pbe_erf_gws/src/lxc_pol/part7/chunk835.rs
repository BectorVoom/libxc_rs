//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 835/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk835<F: Float>(t17140: F, t211: F, t5098: F, t582: F, t1655: F, t4991: F, t587: F, t5351: F, t586: F, t645: F, t1648: F, t4992: F, t4995: F, t1678: F, t1773: F, t184: F, t199: F) -> (F, F, F, F, F, F, F) {
    let t17141 = 128.0 / 45.0 * t17140;
    let t17143 = t211 * t582 * t5098;
    let t17144 = 16.0 / 45.0 * t17143;
    let t17146 = t587 * t4991 * t1655;
    let t17147 = 16.0 / 135.0 * t17146;
    let t17148 = t5351 * t586;
    let t17150 = 32.0 / 15.0 * t17148 * t645;
    let t17151 = t1648 * t4992;
    let t17152 = 32.0 / 135.0 * t17151;
    let t17153 = t1648 * t4995;
    let t17154 = 64.0 / 45.0 * t17153;
    let t17158 = 8.0 / 5.0 * t1678 * t1773 * t184 * t199;
    (t17141, t17144, t17147, t17150, t17152, t17154, t17158)
}
