//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 713/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk713(t6359: f64, t6437: f64, t6625: f64, t6627: f64, t6634: f64, t6638: f64, t6640: f64, t6644: f64, t6647: f64, t6694: f64, t6696: f64, t6709: f64, t6737: f64) -> f64 {
    let t6808 = t6625 - t6627 - t6634 - t6638 - t6640 - t6644 - t6647 - t6694 - t6696 - t6709 + t6359 + t6737 - t6437;
    t6808
}
