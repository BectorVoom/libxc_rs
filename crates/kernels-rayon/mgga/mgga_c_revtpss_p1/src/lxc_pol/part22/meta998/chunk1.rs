//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3390/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390(t1634: f64, t52877: f64, t63597: f64, t11299: f64, t2875: f64, t6110: f64, t15101: f64, t15383: f64, t63633: f64, t63636: f64, t63638: f64, t63641: f64, t63644: f64, t63647: f64, t63649: f64, t63653: f64, t63656: f64, t63660: f64, t63662: f64) -> (f64, f64, f64, f64) {
    let t63665 = 0.14035736694323150897e2_f64 * t52877 * t1634 * t63597;
    let t63668 = 24.0_f64 * t11299 * t6110 * t2875;
    let t63670 = 4.0_f64 * t15101 * t15383;
    let t63671 = -t63633 - t63636 - t63638 - t63641 - t63644 - t63647 + t63649 + t63653 - t63656 + t63660 - t63662 - t63665 - t63668 - t63670;
    (t63665, t63668, t63670, t63671)
}
