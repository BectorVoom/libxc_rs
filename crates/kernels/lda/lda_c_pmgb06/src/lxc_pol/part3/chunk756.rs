//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 756/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk756<F: Float>(t3056: F, t3064: F, t4146: F, t4148: F, t4151: F, t5101: F, t5104: F, t5107: F, t5112: F, t5114: F, t5117: F, t5122: F, t5124: F, t5126: F, t5128: F, t5129: F) -> (F, F, F) {
    let t5130 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t3056;
    let t5131 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t3064;
    let t5132 = -F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t4146 + F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t4148 - t4151 + t5101 - t5104 - t5107 + t5112 - t5114 - t5117 + t5122 + t5124 - t5126 - t5128 - t5129 + t5130 - t5131;
    (t5130, t5131, t5132)
}
