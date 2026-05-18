//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 681/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk681<F: Float>(t1161: F, t1701: F, t1705: F, t6360: F, t421: F, t4830: F, t1156: F, t1696: F, t1700: F, t419: F, t1151: F, t418: F) -> (F, F, F, F, F) {
    let t6361 = t1701 * t1161;
    let t6362 = t6361 * t1705;
    let t6363 = t6360 * t6362;
    let t6365 = t421 * t4830;
    let t6367 = F::new(1.28) * t1156 * t6365;
    let t6376 = t1696 * t1701;
    let t6381 = F::new(1.0) / t1700 / t419;
    let t6403 = t1151 * t418;
    (t6363, t6367, t6376, t6381, t6403)
}
