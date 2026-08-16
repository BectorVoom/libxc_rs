//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1074/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1074(t40458: f64, t35707: f64, t35720: f64, t35724: f64, t35731: f64, t35737: f64, t37821: f64, t37822: f64, t37825: f64, t40420: f64, t40425: f64, t40431: f64, t40437: f64, t40442: f64, t40448: f64, t40451: f64, t40456: f64) -> f64 {
    let t43422 = 0.15965655602485078085e0_f64 * t40458;
    let t43428 = 0.1064114997332445985e-4_f64 * t40420 + 0.1064114997332445985e-4_f64 * t40425 - 0.1702583995731913576e-4_f64 * t40431 - 0.1702583995731913576e-4_f64 * t40437 + 0.5107751987195740728e-4_f64 * t40442 - 0.5107751987195740728e-4_f64 * t40448 + 0.1702583995731913576e-4_f64 * t40451 - 0.212822999466489197e-4_f64 * t40456 - t43422 + 0.12195059916630011325e-2_f64 * t35707 + t37821 + t37822 - 0.17347588262831798123e-3_f64 * t35720 - 0.17347588262831798123e-3_f64 * t35724 - t37825 - 0.60975299583150056624e-3_f64 * t35731 - 0.30487649791575028312e-3_f64 * t35737;
    t43428
}
