//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 656/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk656(t418: f64, t92: f64, t422: f64, t93: f64, t108: f64, t1407: f64, t1416: f64, t4352: f64, t4360: f64, t4367: f64, t4373: f64, t476: f64, t478: f64, t726: f64, t728: f64) -> (f64, f64, f64) {
    let t5189 = t92 * t418;
    let t5196 = t93 * t422;
    let t5202 = (40.0_f64 / 27.0_f64 * t476 * t4352 + 20.0_f64 / 3.0_f64 * t5189 * t1407 + 4.0_f64 / 3.0_f64 * t726 * t4360 + 40.0_f64 / 27.0_f64 * t478 * t4367 + 20.0_f64 / 3.0_f64 * t5196 * t1416 + 4.0_f64 / 3.0_f64 * t728 * t4373) * t108;
    (t5189, t5196, t5202)
}
