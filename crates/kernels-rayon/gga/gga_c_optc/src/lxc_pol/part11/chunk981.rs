//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 981/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk981(t17696: f64, t17875: f64, t17896: f64, t17945: f64, t1550: f64, t5351: f64, t4411: f64, t5430: f64, t146: f64, t17926: f64, t455: f64, t17697: f64, t3104: f64) -> (f64, f64, f64, f64, f64) {
    let t17947 = t17696 + t17875 + t17896 + t17945;
    let t17960 = t5351 * t1550;
    let t17964 = t4411 * t5430;
    let t17969 = t146 * t455 * t17926;
    let t17978 = t3104 * t17697;
    (t17947, t17960, t17964, t17969, t17978)
}
