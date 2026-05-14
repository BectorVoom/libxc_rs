//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 341/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk341<F: Float>(t1685: F, t1732: F, t1736: F, t1738: F, t1681: F, t305: F, t429: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t1740 = t1685 - 1.5625 * t1732 + t1736 + 1.5625 * t1738;
    let t1741 = t1681 * t1740;
    let t1743 = 0.025613155472356368 * t1681 + 1.0;
    let t1744 = 1.0 / t1743;
    let t1745 = t1744 * t305;
    let t1746 = t1741 * t1745;
    let t1747 = t429 * t68;
    (t1740, t1741, t1743, t1744, t1745, t1746, t1747)
}
