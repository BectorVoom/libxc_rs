//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 266/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk266(t519: f64, t799: f64, t538: f64, t789: f64, t25: f64, t531: f64, t536: f64, t791: f64) -> (f64, f64, f64) {
    let t801 = 4.0_f64 / 45.0_f64 * t519 * t799;
    let t803 = t538 * t789;
    let t806 = -t531 - 0.035991666666666665_f64 * t791 - t536 - 0.006666666666666667_f64 * t25 * t803;
    (t801, t803, t806)
}
