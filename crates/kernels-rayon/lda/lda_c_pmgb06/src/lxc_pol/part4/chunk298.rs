//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 298/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk298(t696: f64, t980: f64, t109: f64, t660: f64, t265: f64, t659: f64, t260: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t982 = 1.1696447245269292_f64 * t696 * t980;
    let t986 = t109 * t660;
    let t990 = t659 * t265;
    let t991 = 1.0_f64 / t990;
    let t992 = t260 * t991;
    let t993 = t666 * t666;
    (t982, t986, t990, t991, t992, t993)
}
