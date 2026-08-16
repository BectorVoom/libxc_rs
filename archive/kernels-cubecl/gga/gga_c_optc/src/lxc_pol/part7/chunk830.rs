//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 830/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk830<F: Float>(t7878: F, t898: F, t893: F, t2586: F, t2649: F, t2612: F, t309: F, t300: F, t2583: F, t2587: F, t3608: F, t7359: F) -> (F, F, F, F, F, F, F, F) {
    let t7879 = t7878 * t898;
    let t7880 = t893 * t7879;
    let t7882 = t2586 * t2649;
    let t7883 = t893 * t7882;
    let t7885 = t2612 * t309;
    let t7886 = t300 * t7885;
    let t7889 = t2583 * t2587;
    let t7891 = t3608 * t7359;
    (t7879, t7880, t7882, t7883, t7885, t7886, t7889, t7891)
}
