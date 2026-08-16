//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1245/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1245(t14654: f64, t8896: f64, t127: f64, t3296: f64, t14666: f64, t431: f64, t5571: f64, t5509: f64, t925: f64, t2061: f64, t5512: f64, t14646: f64, t5592: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14781 = t8896 * t14654;
    let t14783 = t127 * t3296;
    let t14787 = t431 * t5571 * t14666;
    let t14795 = t5509 * t925;
    let t14796 = 2.93808_f64 * t14795;
    let t14797 = t5512 * t2061;
    let t14799 = t5592 * t14646;
    (t14781, t14783, t14787, t14796, t14797, t14799)
}
