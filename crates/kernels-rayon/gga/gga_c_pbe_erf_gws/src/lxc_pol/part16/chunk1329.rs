//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1329/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1329(t53873: f64, t53886: f64, t51771: f64, t52417: f64, t52483: f64, t53846: f64, t53848: f64, t53856: f64, t53862: f64, t53867: f64, t53870: f64, t53876: f64, t53878: f64, t53880: f64, t53884: f64, t8793: f64) -> f64 {
    let t55403 = 7.0_f64 / 576.0_f64 * t53873;
    let t55408 = 119.0_f64 / 3456.0_f64 * t53886;
    let t55409 = -7.0_f64 / 1152.0_f64 * t51771 + t53846 / 12.0_f64 + t53848 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t52483 + t53856 / 192.0_f64 + t8793 * t52417 / 24.0_f64 + t53862 / 96.0_f64 + 5.0_f64 / 96.0_f64 * t53867 - t53870 / 768.0_f64 + t55403 - t53876 / 128.0_f64 - t53878 / 12.0_f64 + t53880 / 8.0_f64 + t53884 / 48.0_f64 + t55408;
    t55409
}
