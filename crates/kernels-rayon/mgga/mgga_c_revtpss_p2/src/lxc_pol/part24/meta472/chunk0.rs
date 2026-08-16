//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1452/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452(t14524: f64, t51297: f64, t136: f64, t2457: f64, t39680: f64, t6022: f64, t10073: f64, t18746: f64, t18742: f64, t10069: f64, t2718: f64, t6041: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62874 = t51297 * t14524;
    let t62907 = t39680 * t6022 * t136 * t2457;
    let t62909 = t10073 * t18746;
    let t62920 = t10073 * t18742;
    let t62922 = t10069 * t18746;
    let t62929 = t2718 * t6041;
    (t62874, t62907, t62909, t62920, t62922, t62929)
}
