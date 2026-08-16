//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 540/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk540(t107: f64, t2060: f64, t247: f64, t2781: f64, t2786: f64, t93: f64) -> f64 {
    let t2789 = 7.0_f64 / 27.0_f64 * t93 * t2781 - 0.06068888888888889_f64 * t2060 + 0.01829167760955153_f64 * t247 - 0.0036147222222222223_f64 * t107 * t2786;
    t2789
}
