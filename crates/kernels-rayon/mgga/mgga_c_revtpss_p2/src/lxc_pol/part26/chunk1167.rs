//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1167/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1167(t7407: f64, t92890: f64, t2061: f64, t22: f64, t25402: f64, t93140: f64, t25310: f64, t26506: f64, t26485: f64, t93364: f64, t2829: f64, t689: f64, t7384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95543 = t92890 * t7407;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = 0.51727911450665971904e-3_f64 * t93140 * t95546;
    let t95551 = t25310 * t26506;
    let t95553 = t93364 * t26485;
    let t95556 = t689 * t7384 * t2829;
    (t95543, t95546, t95548, t95551, t95553, t95556)
}
