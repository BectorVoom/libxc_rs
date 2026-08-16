//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1115/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1115(t14072: f64, t14084: f64, t14055: f64, t14059: f64, t14061: f64, t14065: f64, t14067: f64, t14070: f64, t14074: f64, t14076: f64, t14080: f64, t14086: f64, t14088: f64, t14094: f64, t14097: f64, t14103: f64) -> (f64, f64, f64) {
    let t14229 = 119.0_f64 / 3456.0_f64 * t14072;
    let t14233 = 35.0_f64 / 216.0_f64 * t14084;
    let t14239 = 5.0_f64 / 192.0_f64 * t14055 + 7.0_f64 / 144.0_f64 * t14059 - t14061 / 192.0_f64 - t14065 / 12.0_f64 + t14067 / 192.0_f64 - t14070 / 24.0_f64 + t14229 - t14074 / 384.0_f64 - t14076 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t14080 + t14233 + t14086 / 384.0_f64 + t14088 / 384.0_f64 - t14094 / 48.0_f64 - t14097 / 48.0_f64 + t14103 / 24.0_f64;
    (t14229, t14233, t14239)
}
