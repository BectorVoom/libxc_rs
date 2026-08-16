//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1309/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1309(t21022: f64, t21025: f64, t21028: f64, t21032: f64, t21033: f64, t21034: f64, t21036: f64, t21038: f64, t21042: f64, t21044: f64, t21051: f64, t21056: f64, t21057: f64) -> f64 {
    let t23211 = t21022 + t21025 - t21028 - t21032 + t21033 - t21034 - t21036 - t21038 + t21042 + t21044 + t21051 - t21056 + t21057;
    t23211
}
