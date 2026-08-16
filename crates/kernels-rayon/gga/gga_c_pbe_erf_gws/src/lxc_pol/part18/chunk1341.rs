//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1341/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1341(t14028: f64, t3810: f64, t11480: f64, t4028: f64, t54268: f64, t54272: f64, t54284: f64, t54286: f64, t54290: f64, t57108: f64, t57110: f64, t57112: f64, t57114: f64, t57117: f64, t57119: f64) -> f64 {
    let t57121 = t14028 * t3810;
    let t57123 = t4028 * t11480;
    let t57125 = -t57108 / 384.0_f64 - t57110 / 64.0_f64 - 7.0_f64 / 288.0_f64 * t57112 + 3.0_f64 / 256.0_f64 * t57114 - t57117 / 8.0_f64 + t54268 + t57119 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t57121 + t57123 / 48.0_f64 - t54272 + t54284 - t54286 - t54290;
    t57125
}
