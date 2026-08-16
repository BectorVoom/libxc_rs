//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1377/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1377(t2448: f64, t374: f64, t4232: f64, t107: f64, t110: f64, t11698: f64, t11700: f64, t11708: f64, t11720: f64, t11723: f64, t11726: f64, t11729: f64, t11731: f64, t11733: f64, t15059: f64, t9045: f64, t9061: f64, t9063: f64) -> (f64, f64) {
    let t18095 = t4232 * t2448 * t374;
    let t18127 = 0.3350512821420176_f64 * t11698 + 0.1675256410710088_f64 * t11700 + 0.1675256410710088_f64 * t11708 + 0.1675256410710088_f64 * t11720 - 1.1389037339096726_f64 * t11723 + 0.3891025816905257_f64 * t11726 + 0.039794582218349216_f64 * t11729 + 1.0051538464260528_f64 * t11731 + 1.0051538464260528_f64 * t11733 + 0.42708890021612717_f64 * t107 * t110 * t15059 - 0.1675256410710088_f64 * t9045 - 0.1675256410710088_f64 * t9061 - 0.3350512821420176_f64 * t9063;
    (t18095, t18127)
}
