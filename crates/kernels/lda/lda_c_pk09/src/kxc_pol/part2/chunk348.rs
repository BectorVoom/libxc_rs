//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 348/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk348<F: Float>(t1684: F, t1735: F, t1732: F, t1738: F, t1755: F, t1762: F, t1766: F, t1769: F, t543: F, t452: F, t337: F, t534: F, t10: F, t430: F) -> (F, F, F, F, F, F, F, F) {
    let t1771 = 1.2466946262544771 * t1684;
    let t1773 = 0.41556487541815906 * t1735;
    let t1775 = t1755 - 6.25 * t1762 + t1766 + 6.25 * t1769 + t1771 - 1.2466946262544771 * t1732 + t1773 + 1.2466946262544771 * t1738;
    let t1776 = 1.0 / t543;
    let t1777 = t1775 * t1776;
    let t1778 = t1777 * t452;
    let t1781 = t534 * t337;
    let t1782 = t430 * t10;
    (t1771, t1773, t1775, t1776, t1777, t1778, t1781, t1782)
}
