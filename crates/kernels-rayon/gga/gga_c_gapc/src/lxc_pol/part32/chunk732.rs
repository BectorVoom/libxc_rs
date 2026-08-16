//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 732/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk732(t8326: f64, t8366: f64, t8405: f64, t8439: f64, t8481: f64, t8519: f64, t8555: f64, t8593: f64, t2962: f64, t575: f64, t1010: f64, t1615: f64) -> (f64, f64, f64) {
    let t8596 = t8326 + t8366 + t8405 + t8439 + t8481 + t8519 + t8555 + t8593;
    let t8598 = t2962 * t575;
    let t8601 = t1010 * t1615;
    (t8596, t8598, t8601)
}
