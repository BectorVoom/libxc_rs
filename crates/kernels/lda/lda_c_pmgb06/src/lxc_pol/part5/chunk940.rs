//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 940/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk940<F: Float>(t6600: F, t802: F, t6572: F, t1887: F, t2631: F, t161: F, t166: F, t2623: F, t4801: F, t1908: F, t6127: F, t2002: F, t6788: F, t16184: F, t1972: F, t6509: F) -> (F, F, F, F, F, F, F, F) {
    let t19714 = t802 * t6600 / 5.0;
    let t19716 = t802 * t6572 / 5.0;
    let t19718 = t1887 * t2631 / 5.0;
    let t19722 = t161 * t166 * t4801 * t2623 / 10.0;
    let t19724 = t6127 * t1908 / 15.0;
    let t19726 = 2.0 / 15.0 * t2002 * t6788;
    let t19727 = 2.0 / 15.0 * t16184;
    let t19729 = 8.0 / 27.0 * t1972 * t6509;
    (t19714, t19716, t19718, t19722, t19724, t19726, t19727, t19729)
}
