//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 849/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk849(t5169: f64, t5172: f64, t5208: f64, t2684: f64, t5137: f64, t639: f64, t2571: f64, t4934: f64, t1620: f64, t219: f64, t2591: f64, t2705: f64, t617: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7184 = 16.0_f64 / 45.0_f64 * t5169;
    let t7185 = 8.0_f64 / 45.0_f64 * t5172;
    let t7187 = 8.0_f64 / 135.0_f64 * t5208;
    let t7188 = t5137 * t2684;
    let t7190 = 16.0_f64 / 135.0_f64 * t639 * t7188;
    let t7191 = t4934 * t2571;
    let t7193 = 32.0_f64 / 135.0_f64 * t1620 * t7191;
    let t7194 = t2591 * t219;
    let t7195 = t2705 * t617;
    (t7184, t7185, t7187, t7190, t7193, t7194, t7195)
}
