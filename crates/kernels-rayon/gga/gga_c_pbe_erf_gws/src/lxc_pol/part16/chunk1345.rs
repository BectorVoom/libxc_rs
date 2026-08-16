//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1345/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1345(t54397: f64, t54401: f64, t51461: f64, t51466: f64, t51473: f64, t51479: f64, t52715: f64, t54391: f64, t54394: f64, t54404: f64, t54406: f64, t54408: f64, t54411: f64) -> f64 {
    let t55633 = 7.0_f64 / 72.0_f64 * t54397;
    let t55634 = 7.0_f64 / 72.0_f64 * t54401;
    let t55640 = -t52715 - 7.0_f64 / 24.0_f64 * t51461 - t54391 / 2.0_f64 + 7.0_f64 / 144.0_f64 * t51466 - t54394 / 8.0_f64 + 7.0_f64 / 144.0_f64 * t51473 + t55633 - t55634 - t54404 / 48.0_f64 - t54406 / 192.0_f64 - t54408 / 192.0_f64 + 7.0_f64 / 576.0_f64 * t51479 - t54411 / 48.0_f64;
    t55640
}
