//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 981/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk981<F: Float>(t10: F, t2595: F, t3624: F, t770: F, t2638: F, t311: F, t330: F, t8113: F, t2670: F, t935: F, t297: F, t7380: F) -> (F, F, F, F, F, F, F) {
    let t10959 = t10 * t2595;
    let t10977 = t3624 * t770;
    let t10990 = t2638 * t311;
    let t10991 = t330 * t10990;
    let t11018 = t330 * t8113;
    let t11019 = t2670 * t935;
    let t11020 = t11019 * t297;
    let t11024 = t7380 * t2670;
    (t10959, t10977, t10990, t10991, t11018, t11020, t11024)
}
