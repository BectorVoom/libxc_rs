//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 867/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk867<F: Float>(t1972: F, t1995: F, t1600: F, t2623: F, t529: F, t1992: F, t493: F, t1963: F, t2002: F, t165: F, t842: F, t1994: F) -> (F, F, F, F, F, F, F, F) {
    let t6111 = F::new(2.0) / F::new(15.0) * t1972 * t1995;
    let t6112 = t1600 * t2623;
    let t6113 = t6112 * t529;
    let t6114 = t1992 * t6113;
    let t6116 = t493 * t6114 / F::new(15.0);
    let t6118 = F::new(2.0) / F::new(45.0) * t2002 * t1963;
    let t6119 = t165 * t842;
    let t6120 = t6119 * t1994;
    (t6111, t6112, t6113, t6114, t6116, t6118, t6119, t6120)
}
