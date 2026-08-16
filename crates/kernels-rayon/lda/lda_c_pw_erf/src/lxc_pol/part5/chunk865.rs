//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 865/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk865(t3988: f64, t3992: f64, t7630: f64, t7663: f64, t7678: f64, t7682: f64, t7686: f64, t7690: f64, t7694: f64, t7697: f64, t7700: f64, t7704: f64, t7708: f64, t7712: f64, t7715: f64, t7718: f64, t7722: f64, t7726: f64) -> f64 {
    let t8040 = -t3988 + t3992 + t7630 + t7663 + t7678 - t7682 + t7686 + t7690 - t7694 - t7697 + t7700 + t7704 - t7708 - t7712 - t7715 + t7718 + t7722 - t7726;
    t8040
}
