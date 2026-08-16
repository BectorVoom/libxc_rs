//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1191/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1191(t22843: f64, t27940: f64, t22833: f64, t22914: f64, t7264: f64, t22865: f64, t25983: f64, t22860: f64, t94493: f64, t22854: f64, t7271: f64, t22956: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114551 = t27940 * t22843;
    let t114553 = t27940 * t22833;
    let t114564 = t7264 * t22914;
    let t114566 = t25983 * t22865;
    let t114573 = t94493 * t22860;
    let t114575 = t7271 * t22854;
    let t114577 = t7264 * t22956;
    (t114551, t114553, t114564, t114566, t114573, t114575, t114577)
}
