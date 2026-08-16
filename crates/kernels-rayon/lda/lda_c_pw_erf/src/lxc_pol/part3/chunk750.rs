//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 750/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk750(t4818: f64, t951: f64, t3832: f64, t571: f64, t2027: f64, t3794: f64, t789: f64, t944: f64, t1326: f64, t1325: f64, t197: f64, t2176: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4819 = t4818 * t951;
    let t4820 = t3832 * t4819;
    let t4822 = 4.0_f64 / 27.0_f64 * t571 * t4820;
    let t4824 = 16.0_f64 / 45.0_f64 * t3794 * t2027;
    let t4825 = t789 * t944;
    let t4826 = t1326 * t4825;
    let t4828 = 8.0_f64 / 45.0_f64 * t1325 * t4826;
    let t4829 = t2176 * t197;
    (t4819, t4820, t4822, t4824, t4825, t4826, t4828, t4829)
}
