//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 956/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk956(t5948: f64, t5949: f64, t5952: f64, t5954: f64, t7697: f64, t7702: f64, t7708: f64, t7710: f64, t7712: f64, t7715: f64, t7719: f64, t7724: f64, t7740: f64, t7742: f64, t7744: f64, t7749: f64, t7750: f64) -> f64 {
    let t8449 = t5948 + 4.0_f64 / 3.0_f64 * t5949 + t5952 - t7697 + t7702 + t7708 + t7710 - t7712 - t7715 + t7719 - t7724 + 0.22363485482220676312e-1_f64 * t5954 - t7740 + t7742 - t7744 + t7749 + t7750;
    t8449
}
