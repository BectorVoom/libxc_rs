//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 920/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk920(t1210: f64, t1638: f64, t603: f64, t1631: f64, t4196: f64, t4192: f64, t4199: f64, t10: f64, t225: f64, t4231: f64, t602: f64, t245: f64, t4195: f64) -> (f64, f64, f64, f64, f64) {
    let t10697 = 0.019878653761973935_f64 * t1638 * t1210 * t603;
    let t10702 = t1631 * t4196;
    let t10704 = t4192 * t4199;
    let t10709 = 0.4328416544945937_f64 * t602 * t10 * t225 * t4231;
    let t10712 = 0.06709045644666203_f64 * t1638 * t245 * t4195;
    (t10697, t10702, t10704, t10709, t10712)
}
