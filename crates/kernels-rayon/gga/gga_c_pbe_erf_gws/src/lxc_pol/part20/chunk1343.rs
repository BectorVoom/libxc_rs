//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1343/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1343(t15255: f64, t51382: f64, t3799: f64, t4033: f64, t3867: f64, t51388: f64, t51396: f64, t54302: f64, t55582: f64, t57127: f64, t57130: f64, t57132: f64, t57134: f64, t57138: f64, t57140: f64) -> f64 {
    let t57142 = t51382 * t15255;
    let t57144 = t4033 * t3799;
    let t57146 = t4033 * t3867;
    let t57148 = -t57127 / 4.0_f64 + t57130 / 8.0_f64 + t57132 / 48.0_f64 - t57134 / 384.0_f64 - 119.0_f64 / 3456.0_f64 * t51388 - 119.0_f64 / 1728.0_f64 * t51396 + t57138 / 24.0_f64 + t54302 - t55582 - t57140 / 768.0_f64 - 7.0_f64 / 144.0_f64 * t57142 - 7.0_f64 / 48.0_f64 * t57144 + 7.0_f64 / 144.0_f64 * t57146;
    t57148
}
