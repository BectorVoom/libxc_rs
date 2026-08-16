//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 803/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk803(t163: f64, t169: f64, t717: f64, t841: f64, t164: f64, t1896: f64, t1590: f64, t781: f64, t145: f64, t1904: f64) -> (f64, f64, f64, f64) {
    let t5440 = t169 * t717 * t841 * t163;
    let t5442 = t1896 * t164;
    let t5444 = t781 * t1590;
    let t5446 = t145 * t1904;
    (t5440, t5442, t5444, t5446)
}
