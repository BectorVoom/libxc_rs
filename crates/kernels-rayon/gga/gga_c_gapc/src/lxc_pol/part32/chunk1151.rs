//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1151/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1151(t11837: f64, t612: f64, t761: f64, t11918: f64, t29473: f64, t11923: f64, t30158: f64, t3402: f64, t10036: f64, t11872: f64, t11960: f64, t869: f64, t9555: f64) -> (f64, f64, f64, f64, f64) {
    let t34217 = t761 * t612 * t11837;
    let t34219 = t11918 * t29473;
    let t34222 = t3402 * t11923 * t30158;
    let t34224 = t11872 * t10036;
    let t34227 = t869 * t11960 * t9555;
    (t34217, t34219, t34222, t34224, t34227)
}
