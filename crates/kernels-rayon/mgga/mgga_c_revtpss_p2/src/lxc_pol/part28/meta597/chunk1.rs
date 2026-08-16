//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2072/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2072(t1955: f64, t7282: f64, t9656: f64, t25904: f64, t94634: f64, t94640: f64, t281: f64, t555: f64, t93238: f64, t25898: f64, t7303: f64, t25917: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94823 = t1955 * t7282 * t9656;
    let t94842 = t25904 * t94634;
    let t94844 = t25904 * t94640;
    let t94849 = t281 * t93238 * t555;
    let t94851 = t94849 * t25898 * t7303;
    let t94854 = 0.26019841438354088051e-2_f64 * t9303 * t25917;
    (t94823, t94842, t94844, t94849, t94851, t94854)
}
