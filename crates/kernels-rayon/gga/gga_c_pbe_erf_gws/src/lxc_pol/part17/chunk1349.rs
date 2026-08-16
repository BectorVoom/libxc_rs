//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1349/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1349(t54730: f64, t3965: f64, t9299: f64, t52036: f64, t1115: f64, t50932: f64, t52020: f64, t52027: f64, t54707: f64, t54711: f64, t54714: f64, t54717: f64, t54719: f64, t54722: f64, t54724: f64, t54727: f64, t54729: f64, t827: f64) -> f64 {
    let t54731 = 7.0_f64 / 1152.0_f64 * t54730;
    let t54734 = t3965 * t9299;
    let t54737 = 35.0_f64 / 216.0_f64 * t52036;
    let t54738 = -t54707 / 768.0_f64 - t827 * t54711 / 48.0_f64 + t54714 / 24.0_f64 + t54717 - 35.0_f64 / 216.0_f64 * t52020 - 35.0_f64 / 216.0_f64 * t54719 - t54722 / 48.0_f64 - 119.0_f64 / 13824.0_f64 * t54724 + t54727 + t54729 + t54731 - t1115 * t50932 / 96.0_f64 - t54734 / 16.0_f64 + 7.0_f64 / 36.0_f64 * t52027 + t54737;
    t54738
}
