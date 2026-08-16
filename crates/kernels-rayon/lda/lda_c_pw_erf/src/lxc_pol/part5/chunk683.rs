//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 683/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk683(t3183: f64, t3157: f64, t3168: f64, t3176: f64, t5707: f64, t5708: f64, t5709: f64, t5711: f64, t6066: f64, t6068: f64, t6070: f64, t6072: f64, t6073: f64, t6074: f64, t6075: f64, t6076: f64) -> f64 {
    let t6077 = 8.0_f64 * t3183;
    let t6078 = t6066 + t3157 + t5707 - t6068 + t6070 - t5708 - t3168 + t6072 - t5709 + t6073 + t5711 + t3176 + t6074 + t6075 - t6076 - t6077;
    t6078
}
