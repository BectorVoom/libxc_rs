//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 991/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk991(t10350: f64, t10356: f64, t10360: f64, t10362: f64, t10364: f64, t10369: f64, t10371: f64, t10375: f64, t10377: f64, t10382: f64, t10387: f64, t10391: f64, t10396: f64, t10399: f64, t5906: f64, t5912: f64) -> f64 {
    let t11200 = t10350 + t10356 + t10360 + t10362 + t10364 - t10369 + t10371 + t10375 + t10377 + t10382 - t10387 + t10391 + t5906 + 4.0_f64 / 3.0_f64 * t5912 + t10396 - t10399;
    t11200
}
