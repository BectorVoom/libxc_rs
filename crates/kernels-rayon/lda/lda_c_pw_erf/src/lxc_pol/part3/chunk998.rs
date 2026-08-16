//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 998/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk998(t1191: f64, t163: f64, t169: f64, t841: f64, t10766: f64, t10768: f64, t10772: f64, t10775: f64, t10778: f64, t10783: f64, t10787: f64, t10788: f64, t10791: f64, t10793: f64, t10796: f64, t10800: f64, t10802: f64, t10805: f64) -> f64 {
    let t11652 = t169 * t1191 * t841 * t163;
    let t11661 = 0.0878110494085338_f64 * t11652 - t10766 - 0.01185233419734569_f64 * t10768 - 0.0014862827083471494_f64 * t10772 - 0.01777850129601853_f64 * t10775 - 0.004458848125041448_f64 * t10778 - t10783 - t10787 - 0.07769863529371063_f64 * t10788 - t10791 - t10793 - 0.001975389032890948_f64 * t10796 + t10800 + t10802 + 0.01975389032890948_f64 * t10805;
    t11661
}
