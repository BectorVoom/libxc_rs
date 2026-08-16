//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 997/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk997(t3724: f64, t3758: f64, t696: f64, t963: f64, t3729: f64, t971: f64, t3725: f64, t683: f64, t978: f64, t3741: f64, t957: f64, t3738: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8759 = 69.26343642272586_f64 * t696 * t963 * t3758 * t3724;
    let t8760 = t971 * t3729;
    let t8762 = t971 * t3725;
    let t8769 = 4.678578898107717_f64 * t696 * t978 * t3758 * t683;
    let t8771 = t3741 * t957;
    let t8774 = 6152.411314929844_f64 * t696 * t3738 * t964 * t8771;
    (t8759, t8760, t8762, t8769, t8771, t8774)
}
