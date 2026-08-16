//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1166/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1166(t2118: f64, t7690: f64, t2110: f64, t7700: f64, t2167: f64, t7560: f64, t1455: f64, t8909: f64, t33316: f64, t575: f64, t33338: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t124413 = t7690 * t2118;
    let t124418 = t2110 * t7700;
    let t124420 = t2167 * t7560;
    let t124429 = t1455 * t8909;
    let t124431 = t33316 * t575;
    let t124435 = t571 * t33338;
    (t124413, t124418, t124420, t124429, t124431, t124435)
}
