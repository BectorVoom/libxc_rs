//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 905/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk905<F: Float>(t3453: F, t486: F, t161: F, t3004: F, t530: F, t3450: F, t1554: F, t1640: F, t1603: F, t3457: F, t496: F, t1382: F, t3223: F) -> (F, F, F, F, F, F, F) {
    let t9887 = t486 * t3453;
    let t9890 = t161 * t3004 * t530;
    let t9892 = t486 * t3450;
    let t9895 = t161 * t1554 * t1640;
    let t9898 = t161 * t1554 * t1603;
    let t9908 = t496 * t3457;
    let t9921 = t3223 * t1382;
    (t9887, t9890, t9892, t9895, t9898, t9908, t9921)
}
