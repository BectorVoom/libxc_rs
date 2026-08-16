//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1051/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1051(t2151: f64, t3734: f64, t4556: f64, t980: f64, t2148: f64, t3711: f64, t959: f64, t3742: f64, t968: f64, t273: f64, t4515: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11157 = t2151 * t3734;
    let t11160 = t4556 * t980;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11171 = t4515 * t273 * t698;
    (t11157, t11160, t11162, t11164, t11166, t11168, t11171)
}
