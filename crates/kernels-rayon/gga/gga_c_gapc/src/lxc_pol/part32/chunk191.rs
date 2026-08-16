//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 191/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk191(t203: f64, t505: f64, t618: f64, t197: f64, t128: f64, t190: f64) -> (f64, f64, f64, f64) {
    let t662 = t203 * t505;
    let t663 = t618 * t662;
    let t664 = t197 * t663;
    let t667 = t190 * t128;
    (t662, t663, t664, t667)
}
