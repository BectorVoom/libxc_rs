//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 998/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk998<F: Float>(t2531: F, t5569: F, t1336: F, t2636: F, t1625: F, t2621: F, t741: F, t623: F, t1397: F, t2520: F, t1240: F, t93: F) -> (F, F, F, F, F) {
    let t10774 = t2531 * t5569;
    let t10776 = t2636 * t1336;
    let t10777 = t10776 * t1625;
    let t10779 = t741 * t2621;
    let t10780 = t10779 * t623;
    let t10786 = t2520 * t1397;
    let t10790 = t2520 * t1240;
    let t10791 = t93 * t10790;
    (t10774, t10777, t10780, t10786, t10791)
}
