//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 821/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk821(t2995: f64, t3000: f64, t3009: f64, t3016: f64, t3118: f64, t3121: f64, t3125: f64, t3155: f64, t5694: f64, t5696: f64, t5698: f64, t5699: f64, t5700: f64, t5703: f64, t5704: f64, t5705: f64) -> f64 {
    let t5706 = -t5694 - t5696 + t5698 + t2995 - t3000 - t5699 + t5700 - t3009 - t5703 + t5704 + t3016 + t5705 + t3155 + t3118 - t3121 + t3125;
    t5706
}
