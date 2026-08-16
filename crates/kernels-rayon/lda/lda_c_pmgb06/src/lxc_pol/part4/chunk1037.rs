//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1037/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1037(t1377: f64, t1400: f64, t97: f64, t27: f64, t3027: f64, t545: f64, t1403: f64, t1410: f64, t1767: f64, t184: f64, t186: f64, t30: f64, t32: f64) -> (f64, f64, f64, f64, f64) {
    let t10754 = t1400 * t97 * t1377;
    let t10757 = t3027 * t27 * t545;
    let t10760 = t1403 * t97 * t1377;
    let t10764 = 0.06709045644666203_f64 * t1410 * t97 * t1377;
    let t10769 = 2.8503734567901235e-05_f64 * t184 * t1767 * t30 * t32 * t186;
    (t10754, t10757, t10760, t10764, t10769)
}
