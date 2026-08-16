//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 894/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk894(t3267: f64, t8980: f64, t1664: f64, t299: f64, t732: f64, t1686: f64, t1697: f64, t19: f64, t119: f64, t1568: f64, t473: f64, t1691: f64) -> (f64, f64, f64, f64, f64) {
    let t8985 = t3267 * t8980;
    let t8990 = t732 * t299 * t1664;
    let t8991 = t1686 * t1697 * t19 * t8990;
    let t8994 = t119 * t473 * t1568;
    let t8995 = t1691 * t8994;
    (t8985, t8990, t8991, t8994, t8995)
}
