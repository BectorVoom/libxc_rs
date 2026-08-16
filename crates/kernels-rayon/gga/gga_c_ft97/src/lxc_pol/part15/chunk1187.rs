//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1187/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1187(t10603: f64, t2771: f64, t43574: f64, t43852: f64, t462: f64, t55274: f64, t70826: f64, t70935: f64, t83381: f64, t83385: f64, t83387: f64, t83410: f64, t83463: f64, t83472: f64, t83474: f64, t88269: f64, t89783: f64, t89787: f64) -> f64 {
    let t90379 = 8.0_f64 * t462 * t2771 * t89787 + 8.0_f64 * t462 * t10603 * t89783 - 8.0_f64 / 9.0_f64 * t83381 - 8.0_f64 / 3.0_f64 * t83385 + 8.0_f64 / 9.0_f64 * t83387 - 4.0_f64 / 3.0_f64 * t83410 - 8.0_f64 / 9.0_f64 * t70826 - 4.0_f64 / 3.0_f64 * t83463 + 8.0_f64 / 3.0_f64 * t83472 + 8.0_f64 / 3.0_f64 * t83474 + 16.0_f64 / 9.0_f64 * t70935 + 112.0_f64 / 81.0_f64 * t55274 + t43574 - 80.0_f64 / 81.0_f64 * t462 * t43852 * t88269;
    t90379
}
