//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1196/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1196<F: Float>(t2448: F, t374: F, t4232: F, t107: F, t110: F, t11698: F, t11700: F, t11708: F, t11720: F, t11723: F, t11726: F, t11729: F, t11731: F, t11733: F, t15059: F, t9045: F, t9061: F, t9063: F) -> (F, F) {
    let t18095 = t4232 * t2448 * t374;
    let t18127 = 0.3350512821420176 * t11698 + 0.1675256410710088 * t11700 + 0.1675256410710088 * t11708 + 0.1675256410710088 * t11720 - 1.1389037339096726 * t11723 + 0.3891025816905257 * t11726 + 0.039794582218349216 * t11729 + 1.0051538464260528 * t11731 + 1.0051538464260528 * t11733 + 0.42708890021612717 * t107 * t110 * t15059 - 0.1675256410710088 * t9045 - 0.1675256410710088 * t9061 - 0.3350512821420176 * t9063;
    (t18095, t18127)
}
