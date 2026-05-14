//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1037/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1037<F: Float>(t10603: F, t2771: F, t43574: F, t43852: F, t462: F, t55274: F, t70826: F, t70935: F, t83381: F, t83385: F, t83387: F, t83410: F, t83463: F, t83472: F, t83474: F, t88269: F, t89783: F, t89787: F) -> (F,) {
    let t90379 = 8.0 * t462 * t2771 * t89787 + 8.0 * t462 * t10603 * t89783 - 8.0 / 9.0 * t83381 - 8.0 / 3.0 * t83385 + 8.0 / 9.0 * t83387 - 4.0 / 3.0 * t83410 - 8.0 / 9.0 * t70826 - 4.0 / 3.0 * t83463 + 8.0 / 3.0 * t83472 + 8.0 / 3.0 * t83474 + 16.0 / 9.0 * t70935 + 112.0 / 81.0 * t55274 + t43574 - 80.0 / 81.0 * t462 * t43852 * t88269;
    (t90379,)
}
