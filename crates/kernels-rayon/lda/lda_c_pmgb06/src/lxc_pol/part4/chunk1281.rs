//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1281/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1281(t4941: f64, t831: f64, t13094: f64, t4803: f64, t132: f64, t443: f64, t4828: f64, t814: f64, t1420: f64, t6245: f64, t12063: f64, t439: f64, t805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16840 = 2.0_f64 / 15.0_f64 * t831 * t4941;
    let t16841 = 4.0_f64 / 45.0_f64 * t13094;
    let t16843 = 2.0_f64 / 15.0_f64 * t831 * t4803;
    let t16847 = 4.0_f64 / 45.0_f64 * t132 * t4828 * t814 * t443;
    let t16849 = 2.0_f64 / 45.0_f64 * t1420 * t6245;
    let t16852 = 2.0_f64 / 45.0_f64 * t439 * t12063 * t805;
    (t16840, t16841, t16843, t16847, t16849, t16852)
}
