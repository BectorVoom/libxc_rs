//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 546/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk546(t1152: f64, t1354: f64, t2841: f64, t1769: f64, t421: f64, t1179: f64, t398: f64, t419: f64, t136: f64, t409: f64) -> (f64, f64, f64, f64, f64) {
    let t2844 = 0.0014862827083471494_f64 * t1152 * t2841 * t1354;
    let t2846 = 0.025899545097903542_f64 * t1769 * t421;
    let t2847 = t1179 * t398;
    let t2849 = t2847 * t419 * t421;
    let t2851 = t409 * t136;
    (t2844, t2846, t2847, t2849, t2851)
}
