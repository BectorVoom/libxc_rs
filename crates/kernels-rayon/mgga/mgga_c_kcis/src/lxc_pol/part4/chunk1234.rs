//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1234/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1234(t2046: f64, t3801: f64, t4170: f64, t4160: f64, t2011: f64, t4136: f64, t12322: f64, t1395: f64, t1464: f64, t3728: f64, t5882: f64, t2001: f64, t3954: f64) -> (f64, f64, f64, f64, f64) {
    let t15925 = t2046 * t3801;
    let t15926 = t4170 * t15925;
    let t15927 = t4160 * t15926;
    let t15929 = t2011 * t4136;
    let t15930 = t12322 * t15929;
    let t15931 = t1395 * t15930;
    let t15932 = t1464 * t15931;
    let t15934 = t3728 * t5882;
    let t15936 = t2001 * t3954;
    (t15927, t15929, t15932, t15934, t15936)
}
