//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 462/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk462<F: Float>(t2730: F, t497: F, t489: F, t1937: F, t1939: F, t2733: F, t2736: F, t337: F, t430: F, t1905: F, t2149: F, t309: F) -> (F, F, F, F, F, F) {
    let t2769 = t497 * t2730;
    let t2772 = t489 * t2730;
    let t2777 = t1937 - 2.0 * t2733 + t1939 + 2.0 * t2736;
    let t2778 = t2777 * t337;
    let t2779 = t2778 * t430;
    let t2783 = t309 * t1905 * t2149;
    (t2769, t2772, t2777, t2778, t2779, t2783)
}
