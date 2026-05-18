//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 962/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk962<F: Float>(t10104: F, t68: F, t334: F, t339: F, t9862: F, t2516: F, t747: F, t1513: F, t2606: F, t5785: F, t304: F, t332: F) -> (F, F, F, F, F, F, F) {
    let t10181 = t10104 * t68;
    let t10182 = t10181 * t334;
    let t10184 = t339 * t9862;
    let t10186 = t747 * t2516;
    let t10187 = t1513 * t10186;
    let t10190 = t2606 * t5785;
    let t10193 = t304 * t332;
    (t10181, t10182, t10184, t10186, t10187, t10190, t10193)
}
