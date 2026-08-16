//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 946/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk946(t4192: f64, t4199: f64, t10: f64, t225: f64, t4231: f64, t602: f64, t1638: f64, t245: f64, t4195: f64, t1634: f64, t635: f64, t1125: f64, t603: f64) -> (f64, f64, f64, f64, f64) {
    let t10704 = t4192 * t4199;
    let t10709 = 0.4328416544945937_f64 * t602 * t10 * t225 * t4231;
    let t10712 = 0.06709045644666203_f64 * t1638 * t245 * t4195;
    let t10715 = 0.04472697096444135_f64 * t1638 * t635 * t1634;
    let t10718 = 0.2244364134416412_f64 * t602 * t1125 * t603;
    (t10704, t10709, t10712, t10715, t10718)
}
