//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1288/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1288<F: Float>(t16925: F, t1423: F, t6124: F, t439: F, t5197: F, t6555: F, t1512: F, t2631: F, t432: F, t6600: F, t1392: F, t2592: F) -> (F, F, F, F, F, F) {
    let t16926 = F::new(4.0) / F::new(135.0) * t16925;
    let t16927 = t1423 * t6124;
    let t16928 = F::new(4.0) / F::new(135.0) * t16927;
    let t16931 = F::new(2.0) / F::new(15.0) * t439 * t5197 * t6555;
    let t16933 = t1512 * t2631 / F::new(15.0);
    let t16935 = F::new(2.0) / F::new(15.0) * t432 * t6600;
    let t16936 = t2592 * t1392;
    (t16926, t16928, t16931, t16933, t16935, t16936)
}
