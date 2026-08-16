//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 739/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk739(t3182: f64, t426: f64, t8193: f64, t8915: f64, t935: f64, t1: f64, t4456: f64, t3107: f64, t3102: f64, t8113: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9114 = t3182 * sigma2;
    let t9115 = t9114 * t426;
    let t9116 = t9115 * t8193;
    let t9117 = t8915 * t935;
    let t9118 = t9117 * t1;
    let t9122 = t4456 * t8193;
    let t9123 = t3107 * t935;
    let t9124 = t9123 * t1;
    let t9128 = t3102 * t8113;
    (t9114, t9115, t9116, t9117, t9118, t9122, t9123, t9124, t9128)
}
