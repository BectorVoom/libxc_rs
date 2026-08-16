//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1336/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1336(t54166: f64, t51256: f64, t54158: f64, t54160: f64, t54162: f64, t54164: f64, t54168: f64, t54170: f64, t54173: f64, t54175: f64, t54177: f64, t54179: f64) -> f64 {
    let t55508 = 7.0_f64 / 72.0_f64 * t54166;
    let t55516 = -t54158 / 24.0_f64 - t54160 / 12.0_f64 - t54162 / 96.0_f64 + t54164 / 48.0_f64 + t55508 + t54168 / 12.0_f64 + t54170 / 24.0_f64 + 7.0_f64 / 72.0_f64 * t51256 - t54173 / 48.0_f64 + 5.0_f64 / 96.0_f64 * t54175 + t54177 / 48.0_f64 - t54179 / 32.0_f64;
    t55516
}
