//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1349/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1349(t3117: f64, t8914: f64, t438: f64, t935: f64, t1028: f64, t19: f64, t3105: f64, t3145: f64, t2849: f64, t3107: f64, t123: f64, t1897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26880 = t3117 * t8914;
    let t26881 = t935 * t438;
    let t26882 = t26881 * t1028;
    let t26887 = t3145 * t3105 * t19;
    let t26888 = t3107 * t2849;
    let t26889 = t1897 * t123;
    (t26880, t26881, t26882, t26887, t26888, t26889)
}
