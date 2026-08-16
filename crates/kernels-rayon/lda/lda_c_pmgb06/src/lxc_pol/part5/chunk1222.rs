//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1222/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1222(t607: f64, t7970: f64, t19718: f64, t19722: f64, t19724: f64, t19726: f64, t19727: f64, t19729: f64, t19731: f64, t19733: f64, t19736: f64, t19738: f64, t19739: f64) -> f64 {
    let t21928 = t7970 * t607;
    let t21930 = -t19718 - t19722 - 2.0_f64 / 45.0_f64 * t21928 - t19724 - t19726 - t19727 + t19729 - t19731 - t19733 - t19736 - t19738 - t19739;
    t21930
}
