//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 453/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk453(t1738: f64, t286: f64, t1189: f64, t1195: f64, t125: f64, t143: f64, t1547: f64, t1550: f64, t1556: f64, t1569: f64, t1650: f64, t1664: f64, t1727: f64, t1729: f64, t1733: f64, t1735: f64, t279: f64, t405: f64, t453: f64, t456: f64) -> (f64, f64) {
    let t1740 = 0.05321881782335382_f64 * t1738 * t286;
    let t1741 = 6.0_f64 * t1729 * t143 * t1664 + t1650 * t125 + t1547 * t279 + t453 * t1550 - t453 * t1556 + 3.0_f64 * t405 * t1569 + t1727 * t456 + 6.0_f64 * t1733 * t1735 - t1189 + t1195 - t1740;
    (t1740, t1741)
}
