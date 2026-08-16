//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1023/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1023(t2362: f64, t2379: f64, t2408: f64, t3079: f64, t3207: f64, t335: f64, t6156: f64, t6173: f64, t6793: f64, t6797: f64, t8654: f64, t8776: f64, t8780: f64, t8784: f64, t8790: f64, t8793: f64, t8797: f64, t8803: f64, t8806: f64, t8810: f64, t8812: f64, t8818: f64, t9203: f64, t9208: f64) -> f64 {
    let t9211 = -t8776 * t2362 / 32.0_f64 + t8780 + t8784 * t3079 / 96.0_f64 + t6793 * t8790 / 24.0_f64 + t8793 * t6797 / 24.0_f64 + t2408 * t8797 / 24.0_f64 - t8803 + t3207 * t8806 / 8.0_f64 - t8810 + t2408 * t8812 / 24.0_f64 + 7.0_f64 / 288.0_f64 * t6156 - t8654 * t2379 / 48.0_f64 - 35.0_f64 / 432.0_f64 * t8818 + t335 * t9203 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t6173 - t335 * t9208 / 96.0_f64;
    t9211
}
