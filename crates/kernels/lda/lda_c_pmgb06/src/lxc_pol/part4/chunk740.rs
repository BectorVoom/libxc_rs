//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 740/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk740<F: Float>(t1836: F, t489: F, t161: F, t1933: F, t486: F, t1874: F, t432: F, t2095: F, t1641: F, t831: F, t1835: F, t517: F) -> (F, F, F, F, F, F, F) {
    let t4790 = t489 * t1836;
    let t4792 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t161 * t4790;
    let t4794 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t486 * t1933;
    let t4796 = t432 * t1874 / F::cast_from(15.0_f64);
    let t4798 = t486 * t2095 / F::cast_from(15.0_f64);
    let t4800 = t831 * t1641 / F::cast_from(15.0_f64);
    let t4801 = t1835 * t517;
    (t4790, t4792, t4794, t4796, t4798, t4800, t4801)
}
