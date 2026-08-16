//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk989(t9104: f64, t9106: f64, t11751: f64, t11756: f64, t11758: f64, t11759: f64, t11760: f64, t11761: f64, t11763: f64, t11766: f64, t11770: f64, t9108: f64) -> (f64, f64, f64, f64) {
    let t11771 = 2.0_f64 / 45.0_f64 * t9104;
    let t11772 = 2.0_f64 / 135.0_f64 * t9106;
    let t11773 = t11751 - t11756 + t11758 + t11759 + t11760 + t11761 - t11763 - t11766 - t11770 + t11771 + t11772;
    let t11774 = 2.0_f64 / 27.0_f64 * t9108;
    (t11771, t11772, t11773, t11774)
}
