//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 838/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk838<F: Float>(t119: F, t2418: F, t3254: F, t7731: F, t155: F, t7991: F, t151: F, t8141: F, t1062: F, t2238: F, t721: F, t1067: F, t2271: F) -> (F, F, F, F, F, F) {
    let t8555 = t2418 * t119;
    let t8560 = t3254 * t7731;
    let t8564 = t155 * t7991;
    let t8566 = t151 * t8141;
    let t8570 = t2238 * t1062;
    let t8571 = t8570 * t721;
    let t8573 = t2271 * t1067;
    (t8555, t8560, t8564, t8566, t8571, t8573)
}
