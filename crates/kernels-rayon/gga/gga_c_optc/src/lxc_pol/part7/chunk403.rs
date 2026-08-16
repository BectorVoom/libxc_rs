//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 403/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk403(t1963: f64, t40: f64, t539: f64, t592: f64, t544: f64, t559: f64, t712: f64, t171: f64, t1: f64, t558: f64, t598: f64, t110: f64, t518: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1964 = t40 * t1963;
    let t1966 = 8.0_f64 * t539 * t592;
    let t1968 = 8.0_f64 * t544 * t592;
    let t1969 = t539 * t559;
    let t1970 = 8.0_f64 * t1969;
    let t1972 = t712 * t712;
    let t1974 = t171 * t171;
    let t1975 = 1.0_f64 / t1974;
    let t1979 = t558 * t1;
    let t1980 = t1979 * t598;
    let t1981 = 0.36623110073506319882e-3_f64 * t1980;
    let t1983 = t518 * t110 * t84;
    (t1964, t1966, t1968, t1970, t1972, t1974, t1975, t1979, t1981, t1983)
}
