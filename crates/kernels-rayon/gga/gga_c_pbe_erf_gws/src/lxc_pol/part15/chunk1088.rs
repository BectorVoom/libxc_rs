//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1088/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1088(t13856: f64, t13862: f64, t13866: f64, t13870: f64, t13873: f64, t13875: f64, t13878: f64, t13881: f64, t13884: f64, t13886: f64, t13890: f64, t13895: f64, t13896: f64, t13900: f64, t13904: f64, t13907: f64, t13911: f64, t2408: f64, t335: f64, t6793: f64) -> f64 {
    let t13914 = -t13856 / 48.0_f64 + t13862 / 384.0_f64 + t13866 / 384.0_f64 - t13870 / 3072.0_f64 + t13873 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t13875 + t13878 / 768.0_f64 - t335 * t13881 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t13884 + 7.0_f64 / 144.0_f64 * t13886 - t2408 * t13890 / 12.0_f64 + t13895 + t13896 / 48.0_f64 - t13900 / 3072.0_f64 + t13904 / 1536.0_f64 + t13907 / 1536.0_f64 + t6793 * t13911 / 24.0_f64;
    t13914
}
