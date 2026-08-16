//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 415/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk415(t1038: f64, t4052: f64, t417: f64, t431: f64, t1037: f64, t176: f64, t1041: f64, t184: f64, t3878: f64, t384: f64, t73: f64, t1008: f64, t294: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4054 = t1038 * t4052 * t417;
    let t4056 = 0.35089341735807877242e1_f64 * t431 * t4054;
    let t4058 = 1.0_f64 / t1037 / t176;
    let t4060 = t4058 * t4052 * t1041;
    let t4062 = 0.10389515463408878255e3_f64 * t431 * t4060;
    let t4064 = 24.0_f64 * t3878 * t184;
    let t4065 = t73 * t384;
    let t4066 = t4065 * t184;
    let t4068 = t294 * t1008;
    (t4056, t4058, t4062, t4064, t4066, t4068)
}
