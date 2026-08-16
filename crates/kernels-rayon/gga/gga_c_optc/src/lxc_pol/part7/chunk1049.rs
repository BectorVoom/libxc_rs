//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1049/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1049(t2074: f64, t6893: f64, t2024: f64, t616: f64, t2067: f64, t1948: f64, t2012: f64, t630: f64, t6560: f64, t6804: f64, t9686: f64, t1928: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22800 = t6893 * t2074;
    let t22806 = t2024 * t616;
    let t22807 = t22806 * t2067;
    let t22811 = t2012 * t1948;
    let t22815 = t630 * t6560;
    let t22819 = t9686 * t6804;
    let t22822 = t6 * t1928 * t1948;
    (t22800, t22807, t22811, t22815, t22819, t22822)
}
