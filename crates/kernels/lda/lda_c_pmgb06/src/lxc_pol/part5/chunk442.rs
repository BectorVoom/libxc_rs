//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 442/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk442<F: Float>(t1017: F, t1021: F, t1028: F, t1038: F, t1046: F, t1093: F, t1095: F, t1107: F, t1114: F, t1115: F, t1124: F, t2142: F, t283: F, t975: F) -> F {
    let t2147 = -F::cast_from(0.00018311447306006544_f64) * t975 - t1021 + t1114 - t1028 + t1038 + t1046 + t1124 + F::cast_from(0.0197516734986138_f64) * t2142 * t283 - t1017 - t1107 - F::cast_from(4.0_f64) * t1115 + t1093 - F::cast_from(4.0_f64) * t1095;
    t2147
}
