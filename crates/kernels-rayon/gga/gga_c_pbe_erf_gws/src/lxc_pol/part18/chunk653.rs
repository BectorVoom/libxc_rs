//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 653/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk653(t1928: f64, t3515: f64, t3517: f64, t3521: f64, t3525: f64, t3529: f64, t3533: f64, t3537: f64, t3538: f64, t3557: f64, t3559: f64, t3561: f64, t3566: f64) -> f64 {
    let t3607 = t1928 - t3515 + t3517 + t3521 + t3525 + t3529 + t3533 - t3537 + t3538 - t3557 - t3559 + t3561 + t3566;
    t3607
}
