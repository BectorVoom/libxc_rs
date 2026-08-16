//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3101/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101(t68262: f64, t68277: f64, t68312: f64, t68332: f64, t68334: f64, t68336: f64, t68368: f64, t68370: f64, t81423: f64, t81425: f64, t81427: f64, t81429: f64) -> f64 {
    let t81705 = -0.5738611111111111111e0_f64 * t68262 - 0.103295e1_f64 * t68277 + 0.104195e0_f64 * t81423 - 0.69463333333333333333e-1_f64 * t81425 + 0.13892666666666666667e0_f64 * t81427 - 0.41678e0_f64 * t81429 + 0.69463333333333333333e-1_f64 * t68312 + 0.34431666666666666666e0_f64 * t68332 + 0.68863333333333333332e0_f64 * t68334 + 0.20658999999999999999e1_f64 * t68336 - 0.41678e0_f64 * t68368 - 0.9261777777777777778e-1_f64 * t68370;
    t81705
}
