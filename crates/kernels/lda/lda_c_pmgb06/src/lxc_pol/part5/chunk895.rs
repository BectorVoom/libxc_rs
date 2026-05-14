//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 895/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk895<F: Float>(t18061: F, t199: F, t566: F, t6946: F, t5522: F, t868: F, t247: F, t1767: F, t1770: F, t2414: F, t419: F, t4359: F, t7081: F, t2695: F, t384: F, t365: F, t5772: F, t6996: F) -> (F, F, F, F, F, F, F, F) {
    let t18430 = t18061 * t199;
    let t18432 = t6946 * t566;
    let t18434 = t5522 * t868;
    let t18436 = 12.0 * t247;
    let t18453 = t1767 * t2414 * t419 * t1770;
    let t18474 = t4359 * t7081;
    let t18481 = t384 * t2695;
    let t18582 = t365 * t6996 * t5772;
    (t18430, t18432, t18434, t18436, t18453, t18474, t18481, t18582)
}
