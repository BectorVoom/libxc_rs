//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 701/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk701(t1924: f64, t193: f64, t1949: f64, t197: f64, t6560: f64, t3575: f64, t6653: f64, t6656: f64, t6660: f64, t750: f64, t201: f64, t5: f64) -> (f64, f64, f64) {
    let t6663 = t193 * t1924 * t1949;
    let t6668 = t197 * t6560;
    let t6672 = t6653 - 2200.0_f64 / 27.0_f64 * t6656 + 200.0_f64 / 9.0_f64 * t6660 + 200.0_f64 / 9.0_f64 * t6663 - 25.0_f64 / 3.0_f64 * t193 * t3575 * t1949 - 25.0_f64 / 9.0_f64 * t193 * t750 * t6668;
    let t6674 = t5 * t6672 * t201;
    (t6668, t6672, t6674)
}
