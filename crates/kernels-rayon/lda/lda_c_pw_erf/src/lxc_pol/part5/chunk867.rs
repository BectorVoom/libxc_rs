//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 867/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk867(t5872: f64, t5874: f64, t7811: f64, t7813: f64, t7818: f64, t7820: f64, t7824: f64, t7826: f64, t7831: f64, t7833: f64, t7835: f64, t7840: f64, t7841: f64, t7842: f64, t7843: f64, t7844: f64, t7846: f64, t7847: f64) -> f64 {
    let t8047 = t7811 - t7813 - t7818 + t7820 + t7824 - t7826 - t7831 + t7833 + t7835 + t7840 - t7841 + t7842 + 4.0_f64 / 3.0_f64 * t5872 - 2.0_f64 / 9.0_f64 * t5874 - t7843 - t7844 - t7846 + t7847;
    t8047
}
