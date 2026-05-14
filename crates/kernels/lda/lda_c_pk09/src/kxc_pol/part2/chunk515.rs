//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 515/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk515<F: Float>(t3453: F, t200: F, t3230: F, t3233: F, t192: F, t2983: F, t179: F, t155: F, t2974: F, t3262: F, t177: F, t733: F, t142: F, t3163: F, t572: F, t720: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3454 = 50.0 * t3453;
    let t3475 = t200 * t3230;
    let t3477 = t200 * t3233;
    let t3483 = t192 * t2983;
    let t3485 = t179 * t2983;
    let t3488 = t155 * t2983;
    let t3490 = t3262 * t2974;
    let t3494 = t177 * t733;
    let t3495 = t3494 * t142;
    let t3497 = 37.27051603526593 * t3495 * t3163;
    let t3498 = t572 * t720;
    (t3454, t3475, t3477, t3483, t3485, t3488, t3490, t3497, t3498)
}
