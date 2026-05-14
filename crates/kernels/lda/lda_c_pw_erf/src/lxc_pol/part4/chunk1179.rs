//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1179/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1179<F: Float>(t17417: F, t1308: F, t571: F, t6285: F, t954: F, t2120: F, t4564: F, t185: F, t514: F, t6567: F, t1460: F, t519: F, t5255: F, t739: F, t230: F, t7280: F) -> (F, F, F, F, F, F) {
    let t17418 = 32.0 / 45.0 * t17417;
    let t17422 = 4.0 / 45.0 * t571 * t1308 * t6285 * t954;
    let t17423 = t2120 * t4564;
    let t17424 = 16.0 / 135.0 * t17423;
    let t17426 = t185 * t514 * t6567;
    let t17427 = 8.0 / 45.0 * t17426;
    let t17431 = 16.0 / 27.0 * t519 * t5255 * t1460 * t739;
    let t17432 = t7280 * t230;
    (t17418, t17422, t17424, t17427, t17431, t17432)
}
