//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 952/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk952<F: Float>(t2002: F, t6259: F, t1989: F, t6134: F, t224: F, t7627: F, t500: F, t132: F, t137: F, t16267: F, t822: F, t1887: F, t2584: F, t16284: F, t16286: F, t16294: F) -> (F, F, F, F, F, F, F, F) {
    let t19958 = 2.0 / 5.0 * t2002 * t6259;
    let t19960 = t6134 * t1989 / 15.0;
    let t19961 = t7627 * t224;
    let t19963 = t19961 * t500 / 45.0;
    let t19967 = t132 * t137 * t16267 * t822 / 10.0;
    let t19969 = t1887 * t2584 / 10.0;
    let t19970 = 2.0 / 15.0 * t16284;
    let t19971 = 2.0 / 15.0 * t16286;
    let t19972 = 2.0 / 15.0 * t16294;
    (t19958, t19960, t19963, t19967, t19969, t19970, t19971, t19972)
}
