//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 865/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk865(t1112: f64, t3720: f64, t1062: f64, t3709: f64, t696: f64, t957: f64, t27: f64, t3933: f64, t693: f64, t273: f64, t698: f64, t3745: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t8663 = t3720 * t1112;
    let t8668 = 623.3709278045327_f64 * t696 * t3709 * t957 * t1062;
    let t8670 = t3933 * t27 * t693;
    let t8673 = t3933 * t273 * t698;
    let t8675 = t3745 * t980;
    (t8663, t8668, t8670, t8673, t8675)
}
