//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 954/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk954(t2798: f64, t2805: f64, t159: f64, t285: f64, t2853: f64, t462: f64, t4120: f64, t477: f64, t1128: f64, t1159: f64, t2872: f64, t695: f64) -> (f64, f64, f64, f64, f64) {
    let t10852 = t2805 * t2798;
    let t10862 = t462 * t2853 * t159 * t285;
    let t10865 = t4120 * t477 * t285;
    let t10868 = t1159 * t1128 * t285;
    let t10872 = 0.0011622696607154768_f64 * t695 * t2872 * t285;
    (t10852, t10862, t10865, t10868, t10872)
}
