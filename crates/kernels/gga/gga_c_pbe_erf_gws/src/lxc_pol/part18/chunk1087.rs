//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1087/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1087<F: Float>(t3089: F, t4386: F, t13972: F, t14443: F, t1123: F, t52033: F, t833: F, t850: F, t14711: F, t8801: F, t13930: F, t26958: F, t14402: F, t892: F, t1105: F, t353: F, t4053: F) -> (F, F, F, F, F, F, F) {
    let t52996 = t4386 * t3089;
    let t53011 = t13972 * t14443;
    let t53012 = 7.0 / 2304.0 * t53011;
    let t53015 = t850 * t1123 * t52033 * t833;
    let t53025 = 7.0 / 24.0 * t8801 * t14711;
    let t53028 = 7.0 / 72.0 * t26958 * t13930;
    let t53034 = t4386 * t892 * t14402;
    let t53042 = t4386 * t353 * t4053 * t1105;
    (t52996, t53012, t53015, t53025, t53028, t53034, t53042)
}
