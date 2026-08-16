//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 568/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk568(t3010: f64, t2761: f64, t2944: f64, t2950: f64, t2952: f64, t2981: f64, t2989: f64, t2991: f64, t2995: f64, t3000: f64, t3003: f64, t3005: f64, t3009: f64) -> f64 {
    let t3011 = 12.0_f64 * t3010;
    let t3012 = -t2761 - t2944 + t2950 + t2952 + t2981 - t2989 - t2991 + t2995 - t3000 - t3003 + t3005 - t3009 + t3011;
    t3012
}
