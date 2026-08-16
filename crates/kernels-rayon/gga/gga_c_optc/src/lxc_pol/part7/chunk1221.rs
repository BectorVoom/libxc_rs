//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1221/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1221(t2629: f64, t530: f64, t862: f64, t2634: f64, t24: f64, t7406: f64, t2623: f64, t7917: f64, t7914: f64, t2640: f64, t7468: f64, t7477: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25194 = t862 * t530 * t2629;
    let t25197 = t862 * t530 * t2634;
    let t25200 = t862 * t24 * t7406;
    let t25202 = t2623 * t7917;
    let t25208 = t2623 * t7914;
    let t25215 = t2640 * t7468 * t7477;
    (t25194, t25197, t25200, t25202, t25208, t25215)
}
