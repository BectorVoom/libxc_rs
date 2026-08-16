//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 827/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk827(t7832: f64, t7856: f64, t518: f64, t166: f64, t161: f64, t2106: f64, t2648: f64, t137: f64, t132: f64, t3391: f64, t3395: f64, t7765: f64, t7766: f64, t7805: f64, t7810: f64, t7815: f64, t7816: f64, t7817: f64, t7818: f64, t7819: f64, t7820: f64, t7821: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7857 = t7832 + t7856;
    let t7858 = t518 * t7857;
    let t7859 = t166 * t7858;
    let t7861 = t161 * t7859 / 30.0_f64;
    let t7862 = t2106 * t2648;
    let t7863 = t137 * t7862;
    let t7865 = t132 * t7863 / 10.0_f64;
    let t7866 = t7765 + t7766 - t7805 - t7810 - t7815 + t3391 + t3395 + t7816 + t7817 + t7818 + t7819 + t7820 - t7821 + t7822 - t7861 - t7865;
    (t7857, t7858, t7859, t7861, t7862, t7863, t7865, t7866)
}
