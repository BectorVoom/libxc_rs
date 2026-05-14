//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 878/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk878<F: Float>(t5115: F, t802: F, t12981: F, t6633: F, t13007: F, t6562: F, t6630: F, t1636: F, t2563: F, t1593: F, t2648: F, t161: F, t489: F, t6460: F, t1554: F, t2554: F) -> (F, F, F, F, F, F, F, F) {
    let t16537 = t802 * t5115;
    let t16542 = t12981 * t6633;
    let t16549 = t13007 * t6562;
    let t16556 = t13007 * t6630;
    let t16558 = t2563 * t1636;
    let t16563 = t1593 * t2648;
    let t16583 = t161 * t489 * t6460;
    let t16593 = t161 * t1554 * t2554;
    (t16537, t16542, t16549, t16556, t16558, t16563, t16583, t16593)
}
