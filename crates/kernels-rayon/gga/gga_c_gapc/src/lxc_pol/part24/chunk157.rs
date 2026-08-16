//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 157/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk157(t520: f64, t522: f64, t429: f64, t472: f64, t152: f64, t203: f64, t101: f64, t9: f64, t22: f64, t423: f64, t431: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t523 = t520 * t522;
    let t526 = t429 * t472;
    let t527 = t152 * t203;
    let t528 = t9 * t101;
    let t532 = t22 * t423;
    let t536 = t431 * t457;
    (t523, t526, t527, t528, t532, t536)
}
