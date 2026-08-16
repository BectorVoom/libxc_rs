//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 174/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk174(t611: f64, t612: f64, t22: f64, t511: f64, t1: f64, t102: f64) -> (f64, f64, f64) {
    let t613 = t611 * t612;
    let t615 = 1.0_f64 / t22 / t511;
    let t616 = t615 * t1;
    let t617 = t616 * t102;
    (t613, t615, t617)
}
