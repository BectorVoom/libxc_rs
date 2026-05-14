//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 936/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk936<F: Float>(t1925: F, t3223: F, t1423: F, t5238: F, t1908: F, t3220: F, t1972: F, t2984: F, t2974: F, t1382: F, t5194: F, t1592: F, t1962: F, t2865: F, t439: F, t1602: F, t1992: F, t2088: F, t3457: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t12621 = t3223 * t1925;
    let t12622 = 2.0 / 135.0 * t12621;
    let t12623 = t1423 * t5238;
    let t12624 = 4.0 / 45.0 * t12623;
    let t12625 = t3220 * t1908;
    let t12626 = 4.0 / 45.0 * t12625;
    let t12628 = t1972 * t2984 / 15.0;
    let t12630 = 2.0 / 15.0 * t1972 * t2974;
    let t12631 = t5194 * t1382;
    let t12632 = 4.0 / 45.0 * t12631;
    let t12633 = t1962 * t1592;
    let t12636 = 2.0 / 15.0 * t439 * t12633 * t2865;
    let t12641 = 3.0 / 5.0 * t493 * t1992 * t3457 * t2088 * t1602;
    (t12622, t12624, t12626, t12628, t12630, t12632, t12636, t12641)
}
