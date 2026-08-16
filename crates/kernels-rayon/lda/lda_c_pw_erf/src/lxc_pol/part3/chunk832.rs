//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 832/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk832(t2281: f64, t668: f64, t267: f64, t3682: f64, t3684: f64, t3706: f64, t4534: f64, t4535: f64, t4549: f64, t4550: f64, t4551: f64, t4552: f64, t4553: f64, t4554: f64, t4555: f64, t4563: f64, t4566: f64, t5806: f64, t5833: f64) -> f64 {
    let t5837 = 4.0_f64 / 45.0_f64 * t2281 * t668;
    let t5838 = -t4534 + t4535 + 2.0_f64 / 135.0_f64 * t5806 + 4.0_f64 / 135.0_f64 * t3682 - 2.0_f64 / 45.0_f64 * t3684 - t3706 - t5833 * t267 / 15.0_f64 - t5837 + t4549 - t4550 - t4551 + t4552 - t4553 - t4554 - t4555 + t4563 - t4566;
    t5838
}
