//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1426/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1426(t1616: f64, t2011: f64, t3909: f64, t36095: f64, t36100: f64, t36103: f64, t36105: f64, t36109: f64, t36111: f64, t36113: f64, t36116: f64, t36119: f64, t36270: f64, t36271: f64, t36275: f64, t36283: f64, t36285: f64, t36288: f64, t38537: f64, t38556: f64, t38689: f64) -> (f64, f64) {
    let t38692 = 2.0_f64 * t1616 * t3909 * t2011;
    let t38693 = -t36095 + t38537 - t36100 - t36103 + t36105 + t38556 - t38689 - t36109 + t36111 - t36113 - t36116 + t36119 + t38692 - t36270 - t36271 - t36275 + t36283 - t36285 + t36288;
    (t38692, t38693)
}
