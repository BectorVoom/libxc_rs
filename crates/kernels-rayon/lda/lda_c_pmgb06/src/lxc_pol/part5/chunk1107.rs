//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1107/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1107(t12691: f64, t20148: f64, t5068: f64, t20152: f64, t5139: f64, t13068: f64, t5138: f64, t13672: f64, t20156: f64, t5069: f64, t1447: f64, t7656: f64) -> (f64, f64, f64, f64, f64) {
    let t20308 = 4.0_f64 / 15.0_f64 * t5068 * t12691 * t20148;
    let t20311 = 2.0_f64 / 5.0_f64 * t5068 * t5139 * t20152;
    let t20314 = 2.0_f64 / 3.0_f64 * t5138 * t13068 * t20152;
    let t20317 = 8.0_f64 / 15.0_f64 * t13672 * t5069 * t20156;
    let t20318 = t1447 * t7656;
    (t20308, t20311, t20314, t20317, t20318)
}
