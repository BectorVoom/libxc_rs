//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 969/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk969(t3562: f64, t5212: f64, t2741: f64, t7945: f64, t1663: f64, t3345: f64, t3518: f64, t5463: f64, t639: f64, t3503: f64, t4991: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t30170 = t5212 * t3562;
    let t30407 = t2741 * t7945;
    let t30455 = t3345 * t1663;
    let t30511 = t639 * t5463 * t3518;
    let t30583 = t587 * t4991 * t3503;
    (t30170, t30407, t30455, t30511, t30583)
}
