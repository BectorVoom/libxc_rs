//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1344/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1344(t1113: f64, t13781: f64, t3306: f64, t3972: f64, t824: f64, t13808: f64, t15151: f64, t12182: f64, t13792: f64, t11378: f64, t53566: f64, t14733: f64, t9917: f64) -> (f64, f64, f64, f64, f64) {
    let t57648 = t3972 * t13781 * t1113 * t824 * t3306;
    let t57652 = t13808 * t15151;
    let t57654 = t13792 * t12182;
    let t57657 = t53566 * t11378;
    let t57661 = t14733 * t9917;
    (t57648, t57652, t57654, t57657, t57661)
}
