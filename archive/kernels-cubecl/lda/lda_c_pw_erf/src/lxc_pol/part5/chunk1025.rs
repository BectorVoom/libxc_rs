//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1025<F: Float>(t2127: F, t5215: F, t2120: F, t4564: F, t185: F, t514: F, t6567: F, t230: F, t7280: F, t4729: F, t795: F, t5184: F) -> (F, F, F, F, F, F) {
    let t17417 = t5215 * t2127;
    let t17423 = t2120 * t4564;
    let t17426 = t185 * t514 * t6567;
    let t17432 = t7280 * t230;
    let t17434 = t795 * t4729;
    let t17436 = t795 * t5184;
    (t17417, t17423, t17426, t17432, t17434, t17436)
}
