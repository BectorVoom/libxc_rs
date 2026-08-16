//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1033/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1033(t5170: f64, t822: f64, t515: f64, t6788: f64, t568: f64, t6611: f64, t565: f64, t6303: f64, t1318: f64, t3899: f64, t6189: f64, t10030: f64, t6753: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17981 = t822 * t5170;
    let t17983 = t6788 * t515;
    let t17985 = t6611 * t568;
    let t18011 = t565 * t6303;
    let t18023 = t1318 * t3899 * t6189;
    let t18025 = t10030 * t6753;
    (t17981, t17983, t17985, t18011, t18023, t18025)
}
