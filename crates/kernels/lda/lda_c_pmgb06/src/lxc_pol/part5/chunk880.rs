//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 880/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk880<F: Float>(t2489: F, t3223: F, t1592: F, t6225: F, t495: F, t6831: F, t132: F, t1547: F, t2649: F, t497: F, t6904: F, t2466: F, t161: F, t489: F, t6905: F, t5326: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t16749 = t3223 * t2489;
    let t16776 = t1592 * t6225;
    let t16794 = t495 * t6831;
    let t16799 = t132 * t1547 * t2649;
    let t16856 = t6904 * t497;
    let t16866 = t3223 * t2466;
    let t16869 = t161 * t489 * t6905;
    let t16875 = t802 * t5326;
    (t16749, t16776, t16794, t16799, t16856, t16866, t16869, t16875)
}
