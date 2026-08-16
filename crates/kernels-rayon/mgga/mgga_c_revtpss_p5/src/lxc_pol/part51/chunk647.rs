//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 647/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk647(t7018: f64, t789: f64, t159: f64, t793: f64, t218: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64, t234: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7020 = 0.9757440539382783019e-2_f64 * t7018 * t789;
    let t7021 = t793 * t159;
    let t7023 = t7021 * t218 * t816;
    let t7024 = 7.0_f64 / 288.0_f64 * t7023;
    let t7025 = t1941 * t228;
    let t7026 = t7025 * t802;
    let t7028 = t64 * t240;
    let t7030 = t234 * t7028 * t243;
    (t7020, t7021, t7024, t7025, t7026, t7028, t7030)
}
