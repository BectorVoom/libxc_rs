//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 446/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk446<F: Float>(t2160: F, t248: F, t1067: F, t1098: F, t1103: F, t2149: F, t2152: F, t2154: F, t2156: F, t2158: F, t961: F, t970: F, t972: F, t982: F) -> (F, F) {
    let t2161 = t248 * t2160;
    let t2163 = -t1098 + t1103 - F::cast_from(0.5848223622634646_f64) * t2149 - F::cast_from(0.00018311447306006544_f64) * t2152 + t1067 - t961 + F::cast_from(4.0_f64) * t2154 - F::cast_from(4.0_f64) * t2156 + t248 * t2158 + t2161 - t970 - F::cast_from(0.5848223622634646_f64) * t972 + t982;
    (t2161, t2163)
}
