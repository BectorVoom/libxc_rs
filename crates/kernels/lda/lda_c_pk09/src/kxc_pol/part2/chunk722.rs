//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 722/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk722<F: Float>(t2319: F, t748: F, t179: F, t7693: F, t161: F, t3507: F, t3512: F, t3514: F, t3527: F, t3529: F, t3534: F, t3536: F, t3538: F, t3555: F, t3559: F, t1124: F, t2406: F) -> (F, F) {
    let t8078 = t748 * t2319;
    let t8080 = t179 * t7693;
    let t8082 = t161 * t7693;
    let t8088 = 0.027433775686566395 * t8078 - 12.423505345088643 * t8080 - 3.2915558116322368 * t8082 + 3.159189221415045 * t3507 - t3512 - t3514 - t3527 - t3529 + 12.992782516386768 * t3534 + 12.992782516386768 * t3536 - 12.992782516386768 * t3538 - t3555 - t3559;
    let t8089 = t1124 * t2406;
    (t8088, t8089)
}
