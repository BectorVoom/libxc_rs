//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 846/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk846(t6263: f64, t739: f64, t1326: f64, t1325: f64, t2171: f64, t2397: f64, t4041: f64, t4215: f64, t4217: f64, t7736: f64, t7740: f64, t7744: f64, t7748: f64, t7751: f64, t7754: f64, t7755: f64, t7757: f64, t7796: f64, t7801: f64, t7805: f64, t7807: f64) -> (f64, f64, f64, f64, f64) {
    let t7808 = t6263 * t739;
    let t7809 = t1326 * t7808;
    let t7811 = 8.0_f64 / 15.0_f64 * t1325 * t7809;
    let t7813 = 8.0_f64 / 15.0_f64 * t2171 * t2397;
    let t7814 = t7736 - t7740 + t7744 + t7748 - t7751 - t7754 + t4041 + t7755 + t4215 + t4217 - t7757 - t7796 + t7801 - t7805 + t7807 + t7811 - t7813;
    (t7808, t7809, t7811, t7813, t7814)
}
