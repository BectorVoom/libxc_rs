//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 654/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk654(t1426: f64, t245: f64, t645: f64, t1433: f64, t656: f64, t1: f64, t1578: f64, t119: f64, t646: f64, t1423: f64, t3862: f64, t3866: f64, t3871: f64, t3875: f64, t3877: f64, t3879: f64, t3882: f64, t3886: f64, t3890: f64, t3898: f64, t3902: f64, t3907: f64, t3908: f64, t3910: f64, t3912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3915 = t245 * t1426;
    let t3917 = (2e-21_f64 as f64) * t645 * t3915;
    let t3919 = 2.0_f64 / 3.0_f64 * t1433 * t656;
    let t3920 = t1578 * t1;
    let t3921 = t119 * t646;
    let t3923 = 0.001515438175925926_f64 * t3920 * t3921;
    let t3924 = t3862 - t3866 + t3871 + t3875 + t3877 + t3879 + t3882 + t3886 + t3890 + t3898 - t3902 - t3907 + 2.0_f64 / 3.0_f64 * t3908 + 4.0_f64 / 3.0_f64 * t3910 + (2e-21_f64 as f64) * t1423 * t3912 + t3917 + t3919 + t3923;
    (t3915, t3917, t3919, t3920, t3921, t3923, t3924)
}
