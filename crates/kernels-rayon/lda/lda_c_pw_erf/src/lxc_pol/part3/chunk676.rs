//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 676/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk676(t159: f64, t285: f64, t4130: f64, t1155: f64, t477: f64, t147: f64, t343: f64) -> (f64, f64, f64) {
    let t4132 = t4130 * t159 * t285;
    let t4136 = 0.004067943812504169_f64 * t1155 * t477 * t285;
    let t4137 = t343 * t147;
    (t4132, t4136, t4137)
}
