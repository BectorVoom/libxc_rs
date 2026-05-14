//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk816<F: Float>(t446: F, t9836: F, t1423: F, t3012: F, t1447: F, t3204: F, t161: F, t3446: F, t489: F, t3453: F, t486: F, t3004: F, t530: F, t3450: F, t1554: F, t1640: F) -> (F, F, F, F, F, F, F, F) {
    let t9837 = t9836 * t446;
    let t9847 = t1423 * t3012;
    let t9853 = t1447 * t3204;
    let t9885 = t161 * t489 * t3446;
    let t9887 = t486 * t3453;
    let t9890 = t161 * t3004 * t530;
    let t9892 = t486 * t3450;
    let t9895 = t161 * t1554 * t1640;
    (t9837, t9847, t9853, t9885, t9887, t9890, t9892, t9895)
}
