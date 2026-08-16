//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3643/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3643(t58209: f64, t58211: f64, t58225: f64, t68456: f64, t68459: f64, t68567: f64, t68570: f64, t68573: f64, t68576: f64, t68578: f64, t68583: f64, t68585: f64, t68588: f64, t68590: f64, t68593: f64) -> f64 {
    let t68936 = -0.11958666666666666667e1_f64 * t68456 + 0.17938e1_f64 * t68459 - 0.10954222222222222222e0_f64 * t68567 + 0.82156666666666666667e-1_f64 * t68570 - 0.54771111111111111112e-1_f64 * t68573 - 0.27385555555555555556e-1_f64 * t68576 + 0.3071625e0_f64 * t68578 - 0.21908444444444444444e0_f64 * t58209 - 0.65725333333333333332e0_f64 * t58211 + 0.73028148148148148147e0_f64 * t58225 + 0.91285185185185185185e-1_f64 * t68583 + 0.18257037037037037037e0_f64 * t68585 + 0.32862666666666666666e0_f64 * t68588 - 0.30428395061728395062e-1_f64 * t68590 - 0.54771111111111111112e-1_f64 * t68593;
    t68936
}
