//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2089/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089(t16060: f64, t7111: f64, t25539: f64, t4924: f64, t16219: f64, t139: f64, t27526: f64, t3252: f64, t4574: f64, t1014: f64, t4579: f64, t1035: f64, t27543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100359 = t7111 * t16060 / 432.0_f64;
    let t100363 = t25539 * t4924 / 162.0_f64;
    let t100365 = t7111 * t16219;
    let t100370 = t27526 * t139 * t3252 * t4574 / 324.0_f64;
    let t100398 = t27526 * t139 * t1014 * t4579 / 216.0_f64;
    let t100431 = t1035 * t27543;
    (t100359, t100363, t100365, t100370, t100398, t100431)
}
