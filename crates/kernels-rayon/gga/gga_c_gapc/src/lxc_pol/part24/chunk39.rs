//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 39/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk39(t73: f64, t105: f64, t107: f64, t108: f64) -> (f64, f64) {
    let t112 = t73 * t73;
    let t114 = 0.19711288999999999999e-2_f64 * t105 * t107 * t108 - 2.0_f64 * t112;
    let t115 = 1.0_f64 / t114;
    (t114, t115)
}
