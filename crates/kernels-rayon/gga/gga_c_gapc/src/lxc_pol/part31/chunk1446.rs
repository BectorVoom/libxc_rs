//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1446/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1446(t12624: f64, t12667: f64, t36092: f64, t36093: f64, t37317: f64, t37318: f64, t37323: f64, t37324: f64, t37325: f64, t37327: f64, t37328: f64, t37330: f64, t38531: f64, t38532: f64, t38534: f64, t38842: f64, t38843: f64, t38844: f64, t38864: f64, t38866: f64, t7: f64) -> f64 {
    let tv4rho2sigma210 = -t36092 + t36093 + t37317 + t7 * (t38864 + t38866) + t38531 - t37318 + t38532 + t12624 + t38534 - t38842 - t37323 + t37324 + t12667 + t37325 - t38843 - t38844 - t37327 + t37328 - t37330;
    tv4rho2sigma210
}
