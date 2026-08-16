//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3179/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3179(t56248: f64, t56252: f64, t56256: f64, t58202: f64, t58207: f64, t58209: f64, t58211: f64, t58214: f64, t58217: f64, t58220: f64, t58223: f64, t58225: f64) -> f64 {
    let t58585 = 0.49293999999999999999e0_f64 * t58202 + 0.99655555555555555554e0_f64 * t56248 + 0.53814e1_f64 * t56252 - 0.35876e1_f64 * t56256 - 0.73028148148148148149e-1_f64 * t58207 - 0.32862666666666666666e0_f64 * t58209 - 0.98587999999999999998e0_f64 * t58211 + 0.43816888888888888889e0_f64 * t58214 + 0.16431333333333333333e0_f64 * t58217 + 0.147882e1_f64 * t58220 + 0.197176e1_f64 * t58223 + 0.5477111111111111111e0_f64 * t58225;
    t58585
}
