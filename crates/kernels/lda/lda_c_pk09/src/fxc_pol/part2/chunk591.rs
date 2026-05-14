//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 591/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk591<F: Float>(t1397: F, t1471: F, t5624: F, t93: F, t334: F, t5031: F, t1216: F, t1458: F, t1435: F, t1538: F, t1510: F, t1431: F, t1507: F, t1487: F, t5308: F, t1301: F) -> (F, F, F, F, F, F, F, F) {
    let t5625 = t1471 * t1397;
    let t5627 = t5624 * t93 * t5625;
    let t5632 = t5031 * t334;
    let t5635 = t1216 * t1458;
    let t5637 = t1538 * t1435;
    let t5639 = t1510 * t1435;
    let t5641 = t1507 * t1431;
    let t5643 = t1487 * t5308;
    let t5646 = t1301 * t1301;
    (t5627, t5632, t5635, t5637, t5639, t5641, t5643, t5646)
}
