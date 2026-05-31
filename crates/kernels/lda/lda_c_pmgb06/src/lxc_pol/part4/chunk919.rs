//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 919/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk919<F: Float>(t2088: F, t2093: F, t166: F, t161: F, t2582: F, t464: F, t477: F, t137: F, t132: F, t2592: F, t479: F, t1912: F, t1972: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6730 = t2093 * t2088;
    let t6731 = t166 * t6730;
    let t6733 = t161 * t6731 / F::cast_from(15.0_f64);
    let t6734 = t2582 * t464;
    let t6735 = t6734 * t477;
    let t6736 = t137 * t6735;
    let t6738 = t132 * t6736 / F::cast_from(30.0_f64);
    let t6740 = t2592 * t479 / F::cast_from(30.0_f64);
    let t6743 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1972 * t1912;
    (t6730, t6731, t6733, t6734, t6735, t6736, t6738, t6740, t6743)
}
