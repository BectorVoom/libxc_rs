//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1051/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1051(t19567: f64, t19595: f64, t5068: f64, t5090: f64, t6390: f64, t1826: f64, t2600: f64, t1821: f64, t5138: f64, t17628: f64, t1893: f64, t5077: f64) -> (f64, f64, f64, f64, f64) {
    let t19596 = t19567 + t19595;
    let t19599 = 4.0_f64 / 15.0_f64 * t5068 * t5090 * t6390;
    let t19602 = 4.0_f64 / 15.0_f64 * t5068 * t2600 * t1826;
    let t19605 = 2.0_f64 / 9.0_f64 * t5138 * t2600 * t1821;
    let t19608 = 2.0_f64 / 5.0_f64 * t5077 * t17628 * t1893;
    (t19596, t19599, t19602, t19605, t19608)
}
