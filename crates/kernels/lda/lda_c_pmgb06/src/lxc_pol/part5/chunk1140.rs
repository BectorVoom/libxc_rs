//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1140/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1140<F: Float>(t2002: F, t6551: F, t1592: F, t7801: F, t1966: F, t439: F, t477: F, t161: F, t489: F, t7617: F, t490: F, t7628: F) -> (F, F, F, F) {
    let t20684 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t2002 * t6551;
    let t20685 = t1592 * t7801;
    let t20689 = t439 * t1966 * t20685 * t477 / F::cast_from(15.0_f64);
    let t20691 = t161 * t489 * t7617;
    let t20692 = t20691 / F::cast_from(45.0_f64);
    let t20693 = t7628 * t490;
    (t20684, t20689, t20692, t20693)
}
