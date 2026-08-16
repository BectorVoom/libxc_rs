//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1277/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1277(t281: f64, t555: f64, t93238: f64, t25898: f64, t7303: f64, t25917: f64, t9303: f64, t10073: f64, t1444: f64, t2029: f64, t25929: f64, t26041: f64, t9664: f64) -> (f64, f64, f64, f64) {
    let t94849 = t281 * t93238 * t555;
    let t94851 = t94849 * t25898 * t7303;
    let t94854 = 0.26019841438354088051e-2_f64 * t9303 * t25917;
    let t94857 = t10073 * t25929 * t2029 * t1444;
    let t94865 = 0.46263278077393568556e-2_f64 * t26041 * t9664;
    (t94851, t94854, t94857, t94865)
}
