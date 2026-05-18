//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 999/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk999<F: Float>(t2448: F, t374: F, t4232: F, t107: F, t410: F, t6104: F, t122: F, t1669: F, t2659: F, t1680: F, t2527: F, t1730: F, t2526: F) -> (F, F, F, F, F) {
    let t18095 = t4232 * t2448 * t374;
    let t18141 = t107 * t410 * t6104;
    let t18144 = t122 * t1669 * t2659;
    let t18225 = t2527 * t1680;
    let t18244 = t2526 * t1730;
    (t18095, t18141, t18144, t18225, t18244)
}
