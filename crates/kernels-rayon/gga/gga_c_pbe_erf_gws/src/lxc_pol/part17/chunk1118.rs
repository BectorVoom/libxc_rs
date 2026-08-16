//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1118/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1118(t3139: f64, t6646: f64, t14101: f64, t14055: f64, t14060: f64, t14061: f64, t14065: f64, t14067: f64, t14070: f64, t14073: f64, t14074: f64, t14076: f64, t14081: f64, t14085: f64, t14086: f64, t14088: f64, t14094: f64, t14097: f64) -> (f64, f64) {
    let t14102 = t3139 * t6646;
    let t14103 = t14101 * t14102;
    let t14105 = 5.0_f64 / 384.0_f64 * t14055 + t14060 - t14061 / 384.0_f64 - t14065 / 24.0_f64 + t14067 / 384.0_f64 - t14070 / 48.0_f64 + t14073 - t14074 / 768.0_f64 - t14076 / 768.0_f64 + t14081 + t14085 + t14086 / 768.0_f64 + t14088 / 768.0_f64 - t14094 / 96.0_f64 - t14097 / 96.0_f64 + t14103 / 48.0_f64;
    (t14102, t14105)
}
