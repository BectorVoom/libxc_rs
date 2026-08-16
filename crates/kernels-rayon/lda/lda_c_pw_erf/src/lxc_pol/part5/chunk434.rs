//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 434/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk434(t1313: f64, t2030: f64, t519: f64, t549: f64, t816: f64) -> (f64, f64, f64) {
    let t2031 = t1313 * t2030;
    let t2033 = 4.0_f64 / 45.0_f64 * t519 * t2031;
    let t2034 = t816 * t549;
    (t2031, t2033, t2034)
}
