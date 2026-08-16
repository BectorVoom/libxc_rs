//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 644/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk644(t504: f64, t944: f64, t348: f64, t1326: f64, t1325: f64, t1310: f64, t1472: f64, t1360: f64, t593: f64, t1308: f64, t571: f64, t1381: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3817 = t944 * t504;
    let t3818 = t3817 * t348;
    let t3819 = t1326 * t3818;
    let t3821 = 8.0_f64 / 15.0_f64 * t1325 * t3819;
    let t3823 = 8.0_f64 / 15.0_f64 * t1472 * t1310;
    let t3824 = t1360 * t593;
    let t3825 = t1308 * t3824;
    let t3827 = 4.0_f64 / 15.0_f64 * t571 * t3825;
    let t3828 = t559 * t1381;
    (t3818, t3819, t3821, t3823, t3824, t3825, t3827, t3828)
}
