//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1093/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1093<F: Float>(t1464: F, t1639: F, t1601: F, t2918: F, t518: F, t1179: F, t132: F, t441: F, t4829: F, t1554: F, t161: F, t2089: F) -> (F, F, F, F, F) {
    let t13053 = t1639 * t1464;
    let t13064 = t1601 * t1464;
    let t13068 = t518 * t2918;
    let t13079 = t132 * t1179 * t441 * t4829;
    let t13087 = t161 * t1554 * t2089;
    (t13053, t13064, t13068, t13079, t13087)
}
