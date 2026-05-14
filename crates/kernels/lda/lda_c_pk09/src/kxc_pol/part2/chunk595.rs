//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 595/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk595<F: Float>(t1560: F, t305: F, t304: F, t5420: F, t1625: F, t309: F, t310: F, t4977: F, t1642: F, t131: F, t623: F, t307: F, t1615: F, t5569: F, t1435: F, t1562: F) -> (F, F, F, F, F, F, F, F) {
    let t5747 = t1560 * t305;
    let t5751 = t304 * t5420;
    let t5752 = t5751 * t1625;
    let t5755 = t309 * t310 * t4977;
    let t5757 = t1642 * t5755 / 6.0;
    let t5759 = t309 * t131 * t623;
    let t5760 = t307 * t5759;
    let t5762 = t1615 * t5569;
    let t5773 = t1562 * t1435;
    (t5747, t5752, t5755, t5757, t5759, t5760, t5762, t5773)
}
