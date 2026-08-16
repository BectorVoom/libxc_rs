//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2168/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2168(t19890: f64, t26309: f64, t236: f64, t6387: f64, t22705: f64, t22852: f64, t550: f64, t19805: f64, t2002: f64, t559: f64, t19986: f64, t22833: f64) -> (f64, f64, f64, f64, f64) {
    let t97310 = t26309 * t19890;
    let t97312 = t236 * t6387;
    let t97315 = t22852 * t22705 * t97312 * t550;
    let t97318 = t19805 * t2002 * t559;
    let t97320 = t22833 * t19986;
    (t97310, t97312, t97315, t97318, t97320)
}
