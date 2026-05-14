//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1288/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1288<F: Float>(t16007: F, t16010: F, t16015: F, t16017: F, t16018: F, t16019: F, t16020: F, t16021: F, t16023: F, t16025: F, t16029: F, t16035: F, t16037: F, t16040: F, t16043: F, t16048: F, t16051: F) -> (F,) {
    let t19122 = t16007 + t16010 - t16015 + t16017 - t16018 - t16019 + t16020 - t16021 - t16023 - t16025 - t16029 + t16035 - t16037 + t16040 + t16043 - t16048 + t16051;
    (t19122,)
}
