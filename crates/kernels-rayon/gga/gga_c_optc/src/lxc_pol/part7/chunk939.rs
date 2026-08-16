//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 939/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk939(t8428: f64, t8950: f64, t6548: f64, t894: f64, t1136: f64, t6554: f64, t464: f64, t8912: f64, t8914: f64, t935: f64, t438: f64, t450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8951 = t8950 * t8428;
    let t8952 = t8951 * t6548;
    let t8953 = t894 * t8952;
    let t8956 = t1136 * t6554;
    let t8957 = t894 * t8956;
    let t8960 = t464 * t8912;
    let t8961 = t8914 * t935;
    let t8962 = t8961 * t438;
    let t8963 = t450 * t8962;
    (t8951, t8952, t8953, t8956, t8957, t8960, t8962, t8963)
}
