//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 955/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk955(t5945: f64, t7639: f64, t7644: f64, t7648: f64, t7650: f64, t7655: f64, t7658: f64, t7662: f64, t7665: f64, t7668: f64, t7672: f64, t7676: f64, t7679: f64, t7682: f64, t7689: f64, t7693: f64) -> f64 {
    let t8446 = t7639 - t7644 + t7648 + t7650 - t7655 + t7658 + t7662 - t7665 - t7668 + t7672 - t7676 - t7679 - t7682 + t7689 + t7693 + 16.0_f64 / 3.0_f64 * t5945;
    t8446
}
