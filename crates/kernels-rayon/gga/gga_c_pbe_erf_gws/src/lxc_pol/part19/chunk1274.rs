//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1274/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1274(t13808: f64, t15200: f64, t11749: f64, t13917: f64, t53447: f64, t14125: f64, t3781: f64, t833: f64, t850: f64, t1076: f64, t1123: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t56126 = t13808 * t15200;
    let t56129 = t13917 * t53447 * t11749;
    let t56142 = t850 * t3781 * t14125 * t833;
    let t56147 = t850 * t1123 * t1076 * t837 * t833;
    (t56126, t56129, t56142, t56147)
}
