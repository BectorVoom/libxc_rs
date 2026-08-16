//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1257/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1257(t11912: f64, t11917: f64, t11918: f64, t11919: f64, t11920: f64, t11921: f64, t11925: f64, t11927: f64, t11929: f64, t11931: f64, t11937: f64, t11939: f64, t9250: f64) -> f64 {
    let t14976 = -t11912 + t11917 - t11918 - t11919 + t11920 - t9250 - t11921 + t11925 + t11927 + t11929 + t11931 + t11937 - t11939;
    t14976
}
