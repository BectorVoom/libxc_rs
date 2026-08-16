//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 663/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk663(t1870: f64, t1872: f64, t5639: f64, t1697: f64, t9: f64, t133: f64, t5506: f64, t5521: f64, t1904: f64, t285: f64, t477: f64, t281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5641 = t1870 * t5639 * t1872;
    let t5651 = t9 * t1697;
    let t5660 = t133 * t5506;
    let t5663 = 1.1495033333333333_f64 * t133 * t5521;
    let t5677 = t1904 * t477 * t285;
    let t5679 = 0.02394846802050922_f64 * t281 * t5677;
    (t5641, t5651, t5660, t5663, t5677, t5679)
}
