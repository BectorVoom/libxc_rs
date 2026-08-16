//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1121/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1121(t7282: f64, t93139: f64, t2028: f64, t3999: f64, t25875: f64, t25894: f64, t25877: f64, t94382: f64, t1955: f64, t9656: f64, t281: f64, t555: f64, t93238: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94701 = t93139 * t7282;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    let t94823 = t1955 * t7282 * t9656;
    let t94849 = t281 * t93238 * t555;
    (t94701, t94763, t94768, t94771, t94823, t94849)
}
