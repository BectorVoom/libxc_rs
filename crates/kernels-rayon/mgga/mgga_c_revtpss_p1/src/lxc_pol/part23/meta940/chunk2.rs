//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3089/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089(t24297: f64, t698: f64, t58225: f64, t68454: f64, t68456: f64, t68538: f64, t68540: f64, t68548: f64, t68550: f64, t68567: f64, t68583: f64, t68585: f64, t68590: f64) -> (f64, f64) {
    let t81539 = t698 * t24297;
    let t81552 = 0.54771111111111111112e-1_f64 * t81539 - 0.65725333333333333332e0_f64 * t68538 - 0.98587999999999999998e0_f64 * t68540 + 0.10954222222222222222e0_f64 * t68548 + 0.32862666666666666666e0_f64 * t68550 - 0.11958666666666666667e1_f64 * t68454 - 0.17938e1_f64 * t68456 - 0.16431333333333333333e0_f64 * t68567 + 0.54771111111111111112e0_f64 * t58225 + 0.27385555555555555555e0_f64 * t68583 + 0.5477111111111111111e0_f64 * t68585 - 0.91285185185185185184e-1_f64 * t68590;
    (t81539, t81552)
}
