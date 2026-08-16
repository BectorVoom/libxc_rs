//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 988/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk988(t4844: f64, t486: f64, t9089: f64, t9091: f64, t9093: f64, t5105: f64, t161: f64, t489: f64, t4953: f64, t132: f64, t137: f64, t2106: f64, t3441: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11757 = t486 * t4844;
    let t11758 = t11757 / 45.0_f64;
    let t11759 = t9089 / 15.0_f64;
    let t11760 = t9091 / 45.0_f64;
    let t11761 = t9093 / 15.0_f64;
    let t11762 = t486 * t5105;
    let t11763 = 2.0_f64 / 15.0_f64 * t11762;
    let t11765 = t161 * t489 * t4953;
    let t11766 = t11765 / 15.0_f64;
    let t11770 = t132 * t137 * t2106 * t3441 / 30.0_f64;
    (t11758, t11759, t11760, t11761, t11763, t11766, t11770)
}
