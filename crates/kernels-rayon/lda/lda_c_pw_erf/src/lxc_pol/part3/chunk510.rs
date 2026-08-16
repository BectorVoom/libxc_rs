//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 510/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk510(t2131: f64, t493: f64, t514: f64, t807: f64, t185: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t2133 = 4.0_f64 / 15.0_f64 * t493 * t2131;
    let t2134 = t514 * t807;
    let t2135 = t185 * t2134;
    let t2136 = 4.0_f64 / 45.0_f64 * t2135;
    let t2137 = t514 * t812;
    (t2133, t2134, t2136, t2137)
}
