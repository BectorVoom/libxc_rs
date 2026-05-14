//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 801/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk801<F: Float>(t2455: F, t4785: F, t1151: F, t2448: F, t1161: F, t1156: F, t5: F, t2962: F, t2964: F, t4837: F, t4842: F, t271: F, t1179: F, t2447: F, t1185: F, t2452: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9637 = t4785 * t2455;
    let t9643 = t1151 * t2448;
    let t9645 = t2448 * t1161;
    let t9646 = t1156 * t9645;
    let t9648 = 2.8538608299684327 * t5;
    let t9649 = 1.1218014519471058 * t2962;
    let t9650 = 8.429687805830326 * t2964;
    let t9651 = 6.964128765746976 * t4837;
    let t9652 = t9648 - t9649 - t9650 + t9651 - t4842;
    let t9653 = t9652 * t271;
    let t9656 = t2447 * t1179;
    let t9657 = t9656 * t1185;
    let t9660 = t1151 * t2452;
    (t9637, t9643, t9646, t9648, t9649, t9650, t9651, t9653, t9657, t9660)
}
