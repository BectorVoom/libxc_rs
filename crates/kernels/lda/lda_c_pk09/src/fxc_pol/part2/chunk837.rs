//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 837/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk837<F: Float>(t9945: F, t9972: F, t1435: F, t2626: F, t142: F, t338: F, t3677: F, t92: F, t9946: F, t3248: F, t9633: F, t6037: F, t1468: F, t2507: F, t1387: F, t1472: F) -> (F, F, F, F, F, F) {
    let t9973 = t9945 + t9972;
    let t9975 = t2626 * t1435;
    let t9977 = t338 * t142;
    let t9978 = t92 * t3677;
    let t9980 = t9977 * t9978 * t9946;
    let t9982 = t3248 * t9633;
    let t9983 = t6037 * t9982;
    let t9985 = t2507 * t1468;
    let t9986 = t9985 * t1387;
    let t9987 = t9986 * t1472;
    (t9973, t9975, t9980, t9983, t9986, t9987)
}
