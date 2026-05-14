//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 356/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk356<F: Float>(t1684: F, t1735: F, t1732: F, t1738: F, t446: F) -> (F, F, F, F, F, F) {
    let t1811 = 11.879313099038017 * t1684;
    let t1813 = 3.959771033012672 * t1735;
    let t1815 = t1811 - 11.879313099038017 * t1732 + t1813 + 11.879313099038017 * t1738;
    let t1816 = t446 * t446;
    let t1817 = t1816 + 1.0;
    let t1818 = 1.0 / t1817;
    let t1819 = t1815 * t1818;
    (t1811, t1813, t1815, t1817, t1818, t1819)
}
