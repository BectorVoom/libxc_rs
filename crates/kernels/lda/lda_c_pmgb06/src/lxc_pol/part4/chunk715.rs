//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 715/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk715<F: Float>(t529: F, t764: F, t337: F, t5069: F, t5068: F, t129: F, t130: F) -> (F, F, F, F) {
    let t5070 = t764 * t529;
    let t5071 = t5070 * t337;
    let t5072 = t5069 * t5071;
    let t5074 = 4.0 / 45.0 * t5068 * t5072;
    let t5075 = t129 * t130;
    (t5071, t5072, t5074, t5075)
}
