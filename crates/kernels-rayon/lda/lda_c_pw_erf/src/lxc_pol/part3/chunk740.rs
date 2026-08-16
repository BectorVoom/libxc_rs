//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 740/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk740(t1329: f64, t4738: f64, t231: f64, t4705: f64, t4707: f64, t4708: f64, t4709: f64, t4710: f64, t4714: f64, t4718: f64, t4719: f64, t4721: f64, t4726: f64, t4728: f64, t4731: f64, t4733: f64, t4734: f64, t4737: f64) -> (f64, f64) {
    let t4740 = 16.0_f64 / 45.0_f64 * t4738 * t1329;
    let t4741 = t4705 + t4707 + t4708 + t4709 - t4710 + 4.0_f64 / 3.0_f64 * t4714 * t231 + t4718 + 4.0_f64 / 3.0_f64 * t4719 + t4721 + t4726 - t4728 + t4731 + t4733 + 8.0_f64 / 3.0_f64 * t4734 + t4737 + t4740;
    (t4740, t4741)
}
