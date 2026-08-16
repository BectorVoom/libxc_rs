//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1143/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1143(t6037: f64, t980: f64, t11135: f64, t11139: f64, t11142: f64, t8747: f64, t8749: f64, t8755: f64, t8759: f64, t8760: f64, t8762: f64, t8769: f64, t8774: f64, t8779: f64, t8783: f64, t8787: f64, t8794: f64, t8798: f64, t8799: f64) -> f64 {
    let t15015 = t6037 * t980;
    let t15019 = -8.0_f64 * t8747 - 8.0_f64 * t8749 - t8755 - t8759 + 7.017868347161575_f64 * t8760 - 103.89515463408878_f64 * t8762 + t8769 - t8774 + t8779 - 480.0_f64 * t8783 - 0.0011393789434848518_f64 * t8787 - t8794 - 24.0_f64 * t11135 - 48.0_f64 * t11139 + 1.1696447245269292_f64 * t15015 - t8798 + 64.0_f64 * t8799 + 7.017868347161575_f64 * t11142;
    t15019
}
