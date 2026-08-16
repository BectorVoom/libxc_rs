//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1233/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1233(t1162: f64, t13917: f64, t3223: f64, t361: f64, t874: f64, t2081: f64, t28672: f64, t3972: f64, t3975: f64, t6472: f64, t13808: f64, t14698: f64) -> (f64, f64, f64) {
    let t53053 = t13917 * t361 * t1162 * t874 * t3223;
    let t53058 = t3972 * t3975 * t28672 * t6472 * t2081;
    let t53060 = t13808 * t14698;
    (t53053, t53058, t53060)
}
