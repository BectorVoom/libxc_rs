//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1234/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1234(t8880: f64, t8902: f64, t8925: f64, t14549: f64, t14552: f64, t14555: f64, t14558: f64, t14561: f64, t8865: f64, t8869: f64, t8873: f64, t8936: f64, t9083: f64) -> (f64, f64, f64, f64) {
    let t14562 = 2.923025_f64 * t8880;
    let t14563 = 5.84605_f64 * t8902;
    let t14564 = 0.48717083333333333_f64 * t8925;
    let t14565 = t8865 - t8869 + t8873 + t14549 + t14552 - t14555 + t14558 + t14561 + t9083 - t14562 + t14563 + t14564 - t8936;
    (t14562, t14563, t14564, t14565)
}
