//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 458/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk458(t2439: f64, t2536: f64, t869: f64, t903: f64, t291: f64, t672: f64, t332: f64, t959: f64) -> (f64, f64, f64, f64) {
    let t2537 = t2439 * t2536;
    let t2542 = t869 * t903;
    let t2545 = t672 * t291;
    let t2546 = t959 * t332;
    (t2537, t2542, t2545, t2546)
}
