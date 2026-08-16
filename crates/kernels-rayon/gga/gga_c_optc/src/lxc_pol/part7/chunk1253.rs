//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1253/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1253(t25836: f64, t2769: f64, t19: f64, t24567: f64, t1659: f64, t2715: f64, t8072: f64, t24550: f64, t953: f64, t2812: f64, t8044: f64, t8143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25877 = t2769 * t25836;
    let t25878 = t24567 * t19;
    let t25883 = t1659 * t25836;
    let t25888 = t8072 * t2715;
    let t25902 = t953 * t24550;
    let t25905 = t2812 * t8143 * t8044;
    (t25877, t25878, t25883, t25888, t25902, t25905)
}
