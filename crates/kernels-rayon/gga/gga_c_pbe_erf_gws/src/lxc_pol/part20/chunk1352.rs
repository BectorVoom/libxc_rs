//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1352/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1352(t51459: f64, t54398: f64, t54402: f64, t57213: f64, t57216: f64, t57219: f64, t57223: f64, t57225: f64, t57227: f64, t57229: f64, t57231: f64, t57233: f64, t57235: f64) -> f64 {
    let t57237 = -t51459 + 7.0_f64 / 576.0_f64 * t57213 + t54398 - t54402 + t57216 / 96.0_f64 - t57219 / 48.0_f64 - t57223 / 96.0_f64 + t57225 / 64.0_f64 + t57227 / 384.0_f64 + t57229 / 48.0_f64 - t57231 / 384.0_f64 + t57233 / 48.0_f64 + 5.0_f64 / 192.0_f64 * t57235;
    t57237
}
