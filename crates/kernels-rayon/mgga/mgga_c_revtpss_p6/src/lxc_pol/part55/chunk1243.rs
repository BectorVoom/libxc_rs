//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1243/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1243(t8697: f64, t8995: f64, t28199: f64, t122647: f64, t27154: f64, t26399: f64, t7735: f64, t28658: f64, t27137: f64, t7359: f64, t28711: f64, t8634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128529 = t8697 * t8995;
    let t128531 = 2.0_f64 * t128529 * t28199;
    let t128533 = 3.0_f64 * t122647 * t27154;
    let t128535 = 2.0_f64 * t26399 * t7735;
    let t128537 = 2.0_f64 * t28658 * t7735;
    let t128539 = 2.0_f64 * t7359 * t27137;
    let t128543 = 2.0_f64 * t8634 * t28711;
    (t128531, t128533, t128535, t128537, t128539, t128543)
}
