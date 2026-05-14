//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 945/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk945<F: Float>(t11679: F, t462: F, t2042: F, t2149: F, t309: F, t6611: F, t463: F, t453: F, t472: F, t2796: F, t6253: F, t11092: F, t7312: F, t471: F, t476: F, t2006: F, t2811: F) -> (F, F, F, F, F, F, F, F) {
    let t11690 = t462 * t11679;
    let t11691 = t11690 * t2042;
    let t11699 = t309 * t6611 * t2149;
    let t11700 = t463 * t11699;
    let t11702 = t453 * t11699;
    let t11704 = t472 * t11699;
    let t11706 = t2796 * t6253;
    let t11708 = t7312 * t11092;
    let t11711 = t471 * t476;
    let t11714 = t2811 * t2006;
    (t11691, t11700, t11702, t11704, t11706, t11708, t11711, t11714)
}
