//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 402/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk402<F: Float>(t1038: F, t4052: F, t417: F, t431: F, t1037: F, t176: F, t1041: F, t184: F, t3878: F, t384: F, t73: F, t1008: F, t294: F, t1328: F, t74: F, t433: F, t959: F) -> (F, F, F, F, F, F, F, F) {
    let t4054 = t1038 * t4052 * t417;
    let t4056 = 0.35089341735807877242e1 * t431 * t4054;
    let t4058 = 1.0 / t1037 / t176;
    let t4060 = t4058 * t4052 * t1041;
    let t4062 = 0.10389515463408878255e3 * t431 * t4060;
    let t4064 = 24.0 * t3878 * t184;
    let t4065 = t73 * t384;
    let t4066 = t4065 * t184;
    let t4068 = t294 * t1008;
    let t4069 = t4068 * t184;
    let t4071 = 1.0 / t1328;
    let t4072 = t74 * t4071;
    let t4074 = 120.0 * t4072 * t184;
    let t4075 = t959 * t433;
    (t4056, t4058, t4062, t4064, t4066, t4069, t4074, t4075)
}
