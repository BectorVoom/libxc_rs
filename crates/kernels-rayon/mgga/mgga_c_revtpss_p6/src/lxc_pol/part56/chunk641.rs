//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 641/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk641(t218: f64, t7021: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64, t234: f64, t243: f64, t807: f64, t1945: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7023 = t7021 * t218 * t816;
    let t7024 = 7.0_f64 / 288.0_f64 * t7023;
    let t7025 = t1941 * t228;
    let t7026 = t7025 * t802;
    let t7028 = t64 * t240;
    let t7030 = t234 * t7028 * t243;
    let t7031 = t807 * t7030;
    let t7032 = 0.14291339372689912324e-4_f64 * t7031;
    let t7033 = t786 * t1945;
    (t7024, t7025, t7026, t7028, t7030, t7032, t7033)
}
