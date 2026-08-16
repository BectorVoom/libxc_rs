//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1220/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1220(t11944: f64, t19679: f64, t19680: f64, t19681: f64, t19682: f64, t19683: f64, t19685: f64, t19687: f64, t19689: f64, t19693: f64, t9408: f64, t11964: f64, t12038: f64, t19694: f64, t19696: f64, t19697: f64, t19698: f64, t9410: f64, t9412: f64, t9417: f64, t9422: f64, t9426: f64, t9429: f64) -> (f64, f64) {
    let t21922 = t19679 + t19680 - t19681 + t19682 - 0.19947266666666666_f64 * t11944 + t19683 + t19685 - t19687 - t19689 - t19693 - t9408;
    let t21925 = t9410 + t9412 - t9417 + 2.0_f64 / 3.0_f64 * t9422 + t11964 + 8.0_f64 / 81.0_f64 * t9426 + t9429 - t12038 - t19694 - t19696 + t19697 + t19698;
    (t21922, t21925)
}
