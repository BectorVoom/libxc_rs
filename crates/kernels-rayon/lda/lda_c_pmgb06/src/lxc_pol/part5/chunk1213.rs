//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1213/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1213(t11810: f64, t11813: f64, t19311: f64, t19313: f64, t19419: f64, t19421: f64, t19423: f64, t19424: f64, t19425: f64, t19426: f64, t19428: f64, t19431: f64) -> f64 {
    let t21901 = 0.004546314527777778_f64 * t11810 - 0.040518518518518516_f64 * t11813 - t19311 - t19313 + t19419 + t19421 + t19423 - t19424 - t19425 + t19426 - t19428 - t19431;
    t21901
}
