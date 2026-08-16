//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 986/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk986(t1135: f64, t868: f64, t11720: f64, t11723: f64, t11726: f64, t11729: f64, t11731: f64, t4209: f64, t795: f64, t9045: f64, t9048: f64, t9050: f64, t9052: f64, t9061: f64) -> f64 {
    let t11733 = t1135 * t868;
    let t11740 = -0.0837628205355044_f64 * t795 * t4209 + 0.2512884616065132_f64 * t11720 - 1.7083556008645087_f64 * t11723 + 0.19455129084526285_f64 * t11726 + 0.05969187332752383_f64 * t11729 + 0.5025769232130264_f64 * t11731 + 0.5025769232130264_f64 * t11733 - 0.5025769232130264_f64 * t9045 + 0.0837628205355044_f64 * t9048 + 0.2512884616065132_f64 * t9050 + 0.2512884616065132_f64 * t9052 - 0.5025769232130264_f64 * t9061;
    t11740
}
