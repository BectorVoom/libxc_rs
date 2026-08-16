//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1278/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1278(t12677: f64, t12681: f64, t12683: f64, t12685: f64, t12689: f64, t12691: f64, t12694: f64, t12698: f64, t12702: f64, t12706: f64, t12709: f64, t12711: f64, t12713: f64) -> f64 {
    let t15028 = t12677 + t12681 + t12683 - t12685 - t12689 - t12691 - t12694 - t12698 + t12702 + t12706 - t12709 + t12711 - t12713;
    t15028
}
