//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1121/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1121(t13115: f64, t13117: f64, t5166: f64, t13079: f64, t13083: f64, t13085: f64, t13087: f64, t13092: f64, t13096: f64, t13098: f64, t13100: f64, t13103: f64, t13106: f64, t13110: f64, t13114: f64) -> (f64, f64) {
    let t13120 = 32.0_f64 / 9.0_f64 * t13115 * t5166 * t13117;
    let t13121 = t13079 + t13083 - t13085 - t13087 - t13092 + t13096 + t13098 + t13100 + t13103 + t13106 + t13110 + t13114 - t13120;
    (t13120, t13121)
}
