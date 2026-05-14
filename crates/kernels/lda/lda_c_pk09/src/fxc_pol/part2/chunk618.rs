//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 618/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk618<F: Float>(t1700: F, t419: F, t1151: F, t418: F, t1696: F, t253: F, t1207: F, t1705: F, t1197: F, t5066: F, t54: F, t439: F) -> (F, F, F, F, F, F) {
    let t6381 = 1.0 / t1700 / t419;
    let t6403 = t1151 * t418;
    let t6409 = t253 * t1696;
    let t6413 = t1207 * t1705;
    let t6442 = t1197 * t1705;
    let t6463 = t5066 * t54;
    let t6464 = t439 * t6463;
    (t6381, t6403, t6409, t6413, t6442, t6464)
}
