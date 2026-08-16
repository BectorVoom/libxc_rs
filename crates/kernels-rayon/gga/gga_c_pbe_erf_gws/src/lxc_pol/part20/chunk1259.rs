//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1259/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1259(t3195: f64, t4033: f64, t4171: f64, t51407: f64, t14046: f64, t3172: f64, t14565: f64, t346: f64, t838: f64, t859: f64, t4142: f64, t51529: f64) -> (f64, f64, f64, f64, f64) {
    let t54377 = t4033 * t3195;
    let t54378 = 7.0_f64 / 72.0_f64 * t54377;
    let t54381 = t51407 * t4171;
    let t54397 = t14046 * t3172;
    let t54398 = 7.0_f64 / 144.0_f64 * t54397;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54402 = 7.0_f64 / 144.0_f64 * t54401;
    let t54427 = t51529 * t4142;
    (t54378, t54381, t54398, t54402, t54427)
}
