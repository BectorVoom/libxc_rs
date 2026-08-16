//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 963/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk963(t1415: f64, t1432: f64, t256: f64, t1427: f64, t3946: f64, t3949: f64, t656: f64, t3933: f64, t1: f64, t3921: f64, t4166: f64, t119: f64, t1426: f64, t3920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11053 = t1415 * t1432 * t256;
    let t11055 = t3946 * t1427;
    let t11057 = t3949 * t656;
    let t11063 = 8.0_f64 / 9.0_f64 * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    let t11069 = 0.006061752703703704_f64 * t3920 * t119 * t1426;
    (t11053, t11055, t11057, t11063, t11065, t11069)
}
