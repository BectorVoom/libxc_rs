//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 864/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk864<F: Float>(t3768: F, t3867: F, t3871: F, t3874: F, t3877: F, t3892: F, t3893: F, t3901: F, t3906: F, t3908: F, t3911: F, t4550: F, t4552: F, t4554: F, t4558: F, t4559: F) -> F {
    let t6097 = F::cast_from(0.02168716260060348_f64) * t4550 + F::cast_from(2.3392894490538585_f64) * t4552 - F::cast_from(34.63171821136293_f64) * t4554 - t4558 - F::cast_from(1.1696447245269292_f64) * t4559 + t3768 + t3892 - t3867 + t3871 - F::new(8.0) * t3893 + t3874 - F::new(8.0) * t3901 + F::new(32.0) * t3906 + F::new(20.0) * t3908 + t3911 + t3877;
    t6097
}
