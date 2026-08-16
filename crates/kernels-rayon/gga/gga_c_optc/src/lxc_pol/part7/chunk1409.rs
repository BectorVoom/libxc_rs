//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1409/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1409(t26220: f64, t26222: f64, t26228: f64, t26233: f64, t26237: f64, t26240: f64, t26242: f64, t26245: f64, t26251: f64, t26455: f64, t26457: f64, t3274: f64, t8443: f64) -> (f64, f64) {
    let t28052 = -t26220 - t26222 + t26228 - t26233 + t26237 + t26240 + t26242 + t26245 - t26251 + t26455 - t26457;
    let t28061 = t3274 * t8443;
    (t28052, t28061)
}
