//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1347/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1347<F: Float>(t1972: F, t4605: F, t5322: F, t6268: F, t11821: F, t806: F, t2007: F, t5187: F, t1886: F, t1980: F, t2012: F, t13727: F) -> (F, F, F, F, F, F) {
    let t17693 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t4605;
    let t17695 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t6268 * t5322;
    let t17697 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t11821 * t806;
    let t17699 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5187 * t2007;
    let t17702 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1886 * t1980 * t2012;
    let t17703 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13727;
    (t17693, t17695, t17697, t17699, t17702, t17703)
}
