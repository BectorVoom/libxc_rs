//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1097/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1097(t1313: f64, t2954: f64, t519: f64, t5220: f64, t5295: f64, t9304: f64, t3677: f64, t789: f64, t9351: f64, t10467: f64, t1996: f64, t3802: f64, t5425: f64) -> (f64, f64, f64, f64, f64) {
    let t12829 = 8.0_f64 / 15.0_f64 * t519 * t1313 * t5220 * t2954;
    let t12831 = t519 * t9304 * t5295;
    let t12832 = 16.0_f64 / 45.0_f64 * t12831;
    let t12836 = 8.0_f64 / 15.0_f64 * t519 * t9351 * t789 * t3677;
    let t12838 = t519 * t10467 * t1996;
    let t12839 = 8.0_f64 / 135.0_f64 * t12838;
    let t12841 = t519 * t3802 * t5425;
    (t12829, t12832, t12836, t12839, t12841)
}
