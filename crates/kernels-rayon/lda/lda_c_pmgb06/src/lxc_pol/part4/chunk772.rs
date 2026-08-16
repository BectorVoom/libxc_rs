//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 772/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk772(t3056: f64, t3064: f64, t4146: f64, t4148: f64, t4151: f64, t5101: f64, t5104: f64, t5107: f64, t5112: f64, t5114: f64, t5117: f64, t5122: f64, t5124: f64, t5126: f64, t5128: f64, t5129: f64) -> (f64, f64, f64) {
    let t5130 = 2.0_f64 / 135.0_f64 * t3056;
    let t5131 = 2.0_f64 / 45.0_f64 * t3064;
    let t5132 = -2.0_f64 / 45.0_f64 * t4146 + 4.0_f64 / 135.0_f64 * t4148 - t4151 + t5101 - t5104 - t5107 + t5112 - t5114 - t5117 + t5122 + t5124 - t5126 - t5128 - t5129 + t5130 - t5131;
    (t5130, t5131, t5132)
}
