//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 908/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk908<F: Float>(t1240: F, t2743: F, t93: F, t6272: F, t2888: F, t902: F, t633: F, t2739: F, t6977: F, t2738: F, t6258: F, t429: F, t6262: F, t1740: F, t309: F, t454: F) -> (F, F, F, F, F) {
    let t11163 = t2743 * t1240;
    let t11164 = t93 * t11163;
    let t11165 = t6272 * t11164;
    let t11167 = t902 * t2888;
    let t11168 = t11167 * t633;
    let t11172 = t2739 * t6977;
    let t11175 = t6258 * t2738;
    let t11176 = t6262 * t429;
    let t11177 = t11175 * t11176;
    let t11179 = t309 * t454 * t1740;
    (t11165, t11168, t11172, t11177, t11179)
}
