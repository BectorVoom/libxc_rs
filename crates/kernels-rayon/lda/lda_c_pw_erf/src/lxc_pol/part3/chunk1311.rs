//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1311/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1311(t14000: f64, t14001: f64, t14002: f64, t14005: f64, t14007: f64, t14010: f64, t14013: f64, t14017: f64, t14020: f64, t14022: f64, t14025: f64, t14029: f64, t14033: f64, t14037: f64) -> f64 {
    let t15117 = t14000 + t14001 + t14002 + t14005 - t14007 - t14010 - t14013 - t14017 - t14020 - t14022 - t14025 - t14029 - t14033 + t14037;
    t15117
}
