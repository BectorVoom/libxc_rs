//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 553/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk553<F: Float>(t2367: F, t866: F, t930: F, t2629: F, t914: F, t2634: F, t2587: F, t953: F, t301: F, t938: F, t873: F) -> (F, F, F, F, F, F, F) {
    let t2800 = t2367 * t866;
    let t2801 = t930 * t2800;
    let t2803 = t914 * t2629;
    let t2806 = t914 * t2634;
    let t2809 = t953 * t2587;
    let t2811 = t938 * t301;
    let t2812 = t2811 * t873;
    (t2800, t2801, t2803, t2806, t2809, t2811, t2812)
}
