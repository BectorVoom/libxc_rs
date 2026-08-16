//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 615/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk615<F: Float>(t2765: F, t4429: F, t1191: F, t169: F, t301: F, t865: F, t39: F, t780: F, t159: F, t285: F, t1549: F, t1809: F) -> (F, F, F, F, F) {
    let t4430 = t2765 * t4429;
    let t4435 = t169 * t1191 * t865 * t301;
    let t4437 = t39 * t780;
    let t4439 = t4437 * t159 * t285;
    let t4441 = t1549 * t1809;
    (t4430, t4435, t4437, t4439, t4441)
}
