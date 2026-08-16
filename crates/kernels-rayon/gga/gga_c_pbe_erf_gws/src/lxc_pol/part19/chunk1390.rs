//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1390/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1390(t52696: f64, t54331: f64, t55596: f64, t55603: f64, t57171: f64, t57174: f64, t57176: f64, t57179: f64, t57182: f64, t57184: f64, t57186: f64, t57188: f64, t57191: f64) -> f64 {
    let t58752 = -t57171 / 384.0_f64 - t57174 / 48.0_f64 + 7.0_f64 / 576.0_f64 * t57176 + t57179 / 8.0_f64 - t55596 - t54331 - t52696 - 7.0_f64 / 192.0_f64 * t57182 - t57184 / 8.0_f64 - t57186 / 8.0_f64 - 35.0_f64 / 288.0_f64 * t57188 - t57191 / 48.0_f64 - t55603;
    t58752
}
