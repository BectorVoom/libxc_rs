//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1364/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1364<F: Float>(t1887: F, t1928: F, t4810: F, t802: F, t14024: F, t14068: F, t1554: F, t161: F, t2624: F, t1512: F, t2650: F, t132: F, t1547: F, t2630: F) -> (F, F, F, F, F, F, F) {
    let t17919 = t1887 * t1928;
    let t17920 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t17919;
    let t17921 = t802 * t4810;
    let t17922 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t17921;
    let t17923 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14024;
    let t17924 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t14068;
    let t17926 = t161 * t1554 * t2624;
    let t17927 = t17926 / F::cast_from(135.0_f64);
    let t17929 = t1512 * t2650 / F::cast_from(30.0_f64);
    let t17931 = t132 * t1547 * t2630;
    (t17920, t17922, t17923, t17924, t17927, t17929, t17931)
}
