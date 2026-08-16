//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 731/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk731(t5: f64, t6877: f64, t6879: f64, t675: f64, t2024: f64, t6888: f64, t696: f64, t2164: f64, t2174: f64, t155: f64, t2157: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7003 = t5 * t6877;
    let t7004 = t7003 * t6879;
    let t7005 = t675 * t7004;
    let t7008 = t7003 * t2024;
    let t7009 = t675 * t7008;
    let t7012 = t696 * t6888;
    let t7015 = t2164 * t2174;
    let t7018 = t155 * t2157 * t652;
    (t7003, t7005, t7009, t7012, t7015, t7018)
}
