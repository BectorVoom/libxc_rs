//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 525/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk525(t1422: f64, t973: f64, t1431: f64, t993: f64, t356: f64, t997: f64, t996: f64) -> (f64, f64, f64, f64) {
    let t4009 = t1422 * t973;
    let t4033 = t1431 * t993;
    let t4037 = t997 * t356;
    let t4038 = t996 * t4037;
    (t4009, t4033, t4037, t4038)
}
