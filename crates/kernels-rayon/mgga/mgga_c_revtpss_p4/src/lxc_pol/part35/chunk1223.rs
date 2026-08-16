//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1223/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1223(t106053: f64, t106061: f64, t106063: f64, t106065: f64, t113214: f64, t113217: f64, t95673: f64, t95674: f64, t95675: f64, t95678: f64, t95680: f64, t99035: f64, t99044: f64, t99050: f64) -> f64 {
    let t115698 = -0.68598428988911579154e-3_f64 * t106053 - 0.68026775414003982662e-1_f64 * t99035 + 0.34299214494455789577e-3_f64 * t106061 + 0.12004725073059526352e-1_f64 * t106063 - 0.24009450146119052704e-1_f64 * t106065 + 0.12196800674228478774e-3_f64 * t99044 - t95673 + 3.0_f64 / 8.0_f64 * t113214 - 35.0_f64 / 36.0_f64 * t99050 - t95674 + t95675 + t95678 - t113217 / 24.0_f64 - t95680;
    t115698
}
