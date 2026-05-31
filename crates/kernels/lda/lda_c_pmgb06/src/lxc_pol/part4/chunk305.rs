//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 305/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk305<F: Float>(t1025: F, t633: F, t1024: F, t942: F, t945: F, t947: F, t951: F, t953: F, t955: F) -> (F, F, F) {
    let t1026 = t1025 * t633;
    let t1028 = F::cast_from(2.0_f64) * t1024 * t1026;
    let t1035 = -F::cast_from(0.4219833333333333_f64) * t942 + F::cast_from(0.8439666666666666_f64) * t945 + F::cast_from(0.3986222222222222_f64) * t947 + F::cast_from(0.06825833333333334_f64) * t951 + F::cast_from(0.13651666666666668_f64) * t953 + F::cast_from(0.1369277777777778_f64) * t955;
    (t1026, t1028, t1035)
}
