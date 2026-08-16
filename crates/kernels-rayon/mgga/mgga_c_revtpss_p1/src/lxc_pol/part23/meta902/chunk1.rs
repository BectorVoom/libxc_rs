//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2880/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2880(t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t76977: f64, t76978: f64, t76980: f64, t76986: f64, t76987: f64, t1544: f64, t23111: f64, t23148: f64, t2403: f64, t2404: f64, t40131: f64, t40137: f64, t50080: f64, t61139: f64, t76999: f64, t77000: f64, t77002: f64, t77003: f64, t77004: f64, t77005: f64) -> (f64, f64) {
    let t77387 = t39807 - t39813 + t76977 - t39818 - t39823 - t76978 + t40084 + t76980 + t76986 + t40088 - t76987;
    let t77400 = 9.0_f64 * t1544 * t2403 * t61139 + 3.0_f64 * t23148 * t2403 * t2404 + 18.0_f64 * t23111 * t50080 - t40131 - t40137 - t76999 + t77000 + t77002 - t77003 + t77004 + t77005;
    (t77387, t77400)
}
