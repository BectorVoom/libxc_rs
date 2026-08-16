//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2034/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2034(t1294: f64, t21471: f64, t26921: f64, t7648: f64, t12627: f64, t7635: f64, t12587: f64, t7669: f64, t2155: f64, t44126: f64, t2028: f64, t27980: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97398 = t21471 * t1294;
    let t97422 = t7648 * t26921;
    let t97475 = t12627 * t7635;
    let t97491 = t7669 * t12587;
    let t97498 = t2155 * t44126;
    let t97676 = t2028 * t27980;
    (t97398, t97422, t97475, t97491, t97498, t97676)
}
