//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 640/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk640(t1442: f64, t3787: f64, t1325: f64, t3726: f64, t3729: f64, t3734: f64, t3737: f64, t3741: f64, t3744: f64, t3747: f64, t3750: f64, t3754: f64, t3759: f64, t3761: f64, t3765: f64, t3767: f64, t3772: f64, t3777: f64, t3782: f64, t3786: f64) -> (f64, f64, f64, f64) {
    let t3788 = t3787 * t1442;
    let t3789 = t1325 * t3788;
    let t3790 = 16.0_f64 / 15.0_f64 * t3789;
    let t3791 = -t3726 + t3729 + t3734 - t3737 + t3741 + t3744 + t3747 - t3750 + t3754 + t3759 + t3761 - t3765 - t3767 - t3772 - t3777 + t3782 - t3786 - t3790;
    (t3788, t3789, t3790, t3791)
}
