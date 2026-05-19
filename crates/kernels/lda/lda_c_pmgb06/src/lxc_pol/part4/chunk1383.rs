//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1383/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1383<F: Float>(t11796: F, t11798: F, t15474: F, t15475: F, t15476: F, t15480: F, t15482: F, t15484: F, t15487: F, t15490: F, t15493: F, t15496: F, t15498: F, t15501: F, t15506: F) -> F {
    let t18168 = -t15474 - t15475 - t15476 - t15480 + t15482 + F::cast_from(0.19947266666666666_f64) * t11796 + F::cast_from(0.13298177777777778_f64) * t11798 - t15484 - t15487 - t15490 - t15493 - t15496 - t15498 - t15501 - t15506;
    t18168
}
