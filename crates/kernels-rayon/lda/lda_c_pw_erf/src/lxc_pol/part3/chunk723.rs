//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 723/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk723(t1627: f64, t1926: f64, t20: f64, t2259: f64, t1639: f64, t3707: f64, t3736: f64, t3749: f64, t3760: f64, t3764: f64, t3785: f64, t3789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4544 = t1926 * t1627;
    let t4546 = t2259 * t20;
    let t4547 = t4546 * t1639;
    let t4549 = 16.0_f64 / 135.0_f64 * t3707;
    let t4550 = 16.0_f64 / 135.0_f64 * t3736;
    let t4551 = 16.0_f64 / 135.0_f64 * t3749;
    let t4552 = 16.0_f64 / 135.0_f64 * t3760;
    let t4553 = 16.0_f64 / 405.0_f64 * t3764;
    let t4554 = 16.0_f64 / 405.0_f64 * t3785;
    let t4555 = 16.0_f64 / 45.0_f64 * t3789;
    (t4544, t4546, t4547, t4549, t4550, t4551, t4552, t4553, t4554, t4555)
}
