//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1152/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1152(t11918: f64, t29473: f64, t11923: f64, t30158: f64, t3402: f64, t10036: f64, t11872: f64, t11960: f64, t869: f64, t9555: f64, t11965: f64, t9741: f64) -> (f64, f64, f64, f64, f64) {
    let t34219 = t11918 * t29473;
    let t34222 = t3402 * t11923 * t30158;
    let t34224 = t11872 * t10036;
    let t34227 = t869 * t11960 * t9555;
    let t34230 = t869 * t11965 * t9741;
    (t34219, t34222, t34224, t34227, t34230)
}
