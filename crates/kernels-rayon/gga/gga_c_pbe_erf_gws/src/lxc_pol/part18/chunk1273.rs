//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1273/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1273(t13808: f64, t15200: f64, t11749: f64, t13917: f64, t53447: f64, t1161: f64, t20154: f64, t3067: f64, t4155: f64, t14125: f64, t3781: f64, t833: f64, t850: f64) -> (f64, f64, f64, f64) {
    let t56126 = t13808 * t15200;
    let t56129 = t13917 * t53447 * t11749;
    let t56133 = t20154 * t3067 * t4155 * t1161;
    let t56142 = t850 * t3781 * t14125 * t833;
    (t56126, t56129, t56133, t56142)
}
