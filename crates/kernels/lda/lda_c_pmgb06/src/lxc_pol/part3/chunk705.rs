//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 705/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk705<F: Float>(t1122: F, t4549: F, t2148: F, t980: F, t968: F, t2142: F, t273: F, t698: F, t959: F, t3768: F, t3867: F, t3871: F, t3874: F, t3892: F, t3893: F, t3895: F, t3899: F, t3904: F, t3908: F, t3911: F) -> (F, F) {
    let t4550 = t4549 * t1122;
    let t4552 = t2148 * t980;
    let t4554 = t2148 * t968;
    let t4556 = t2142 * t273;
    let t4558 = F::cast_from(1.1696447245269292_f64) * t4556 * t698;
    let t4559 = t2148 * t959;
    let t4566 = F::cast_from(0.01084358130030174_f64) * t4550 + F::cast_from(1.1696447245269292_f64) * t4552 - F::cast_from(17.315859105681465_f64) * t4554 - t4558 - F::cast_from(0.5848223622634646_f64) * t4559 + F::new(2.0) * t3768 + t3892 - t3867 + t3871 - F::new(16.0) * t3893 - F::new(4.0) * t3895 - F::new(4.0) * t3899 + t3874 + t3904 + F::new(40.0) * t3908 + t3911;
    (t4556, t4566)
}
