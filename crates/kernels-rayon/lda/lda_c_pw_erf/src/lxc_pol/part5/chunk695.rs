//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 695/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk695(t577: f64, t6205: f64, t181: f64, t784: f64, t184: f64) -> (f64, f64, f64) {
    let t6207 = 4.0_f64 / 45.0_f64 * t6205 * t577;
    let t6208 = t784 * t181;
    let t6209 = t6208 * t184;
    (t6207, t6208, t6209)
}
