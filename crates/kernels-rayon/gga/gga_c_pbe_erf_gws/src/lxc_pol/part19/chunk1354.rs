//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1354/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1354(t54886: f64, t56061: f64, t56067: f64, t56070: f64, t56074: f64, t56077: f64, t56080: f64, t56093: f64, t56098: f64, t56101: f64, t56105: f64, t56107: f64, t56110: f64, t57958: f64, t57972: f64, t827: f64) -> f64 {
    let t57974 = t56061 / 24.0_f64 - t827 * t57958 / 96.0_f64 + t56067 / 192.0_f64 + 5.0_f64 / 192.0_f64 * t56070 - t56074 / 768.0_f64 - t56077 / 96.0_f64 - t56080 / 96.0_f64 - t54886 - t56093 / 48.0_f64 - t56098 / 192.0_f64 - t56101 / 24.0_f64 - t56105 / 24.0_f64 + 7.0_f64 / 72.0_f64 * t56107 - t56110 / 24.0_f64 + 7.0_f64 / 288.0_f64 * t57972;
    t57974
}
