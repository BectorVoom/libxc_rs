//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1187/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1187<F: Float>(t10603: F, t2771: F, t43574: F, t43852: F, t462: F, t55274: F, t70826: F, t70935: F, t83381: F, t83385: F, t83387: F, t83410: F, t83463: F, t83472: F, t83474: F, t88269: F, t89783: F, t89787: F) -> F {
    let t90379 = F::cast_from(8.0_f64) * t462 * t2771 * t89787 + F::cast_from(8.0_f64) * t462 * t10603 * t89783 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t83381 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t83385 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t83387 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t83410 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t70826 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t83463 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t83472 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t83474 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t70935 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t55274 + t43574 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t43852 * t88269;
    t90379
}
