//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1024/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1024(t2086: f64, t56: f64, t111: f64, t166: f64, t6975: f64, t145: f64, t146: f64, t622: f64, t7000: f64, t155: f64, t6165: f64, t693: f64) -> (f64, f64, f64, f64, f64) {
    let t22895 = t56 * t2086;
    let t22896 = t111 * t22895;
    let t22932 = 1.0_f64 / t6975 / t166;
    let t22933 = t145 * t22932;
    let t23013 = t146 * t7000 * t622;
    let t23017 = t155 * t693 * t6165;
    (t22895, t22896, t22933, t23013, t23017)
}
