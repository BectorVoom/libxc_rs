//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1333/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1333(t54087: f64, t54094: f64, t54102: f64, t51244: f64, t54075: f64, t54077: f64, t54080: f64, t54082: f64, t54085: f64, t54092: f64, t54096: f64, t54098: f64) -> f64 {
    let t55467 = 7.0_f64 / 72.0_f64 * t54087;
    let t55469 = 35.0_f64 / 216.0_f64 * t54094;
    let t55473 = 7.0_f64 / 36.0_f64 * t54102;
    let t55474 = -t54075 / 24.0_f64 + t54077 / 384.0_f64 - t54080 / 24.0_f64 + t54082 / 24.0_f64 - t54085 / 24.0_f64 + t55467 - t54092 / 6.0_f64 + t55469 - t54096 / 384.0_f64 + t54098 / 64.0_f64 - 7.0_f64 / 144.0_f64 * t51244 + t55473;
    t55474
}
