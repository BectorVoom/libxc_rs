//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 727/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk727(t1435: f64, t813: f64, t1440: f64, t439: f64, t1423: f64, t1969: f64, t1431: f64, t2002: f64, t1887: f64, t460: f64, t1542: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4619 = t1435 * t813;
    let t4620 = t4619 * t1440;
    let t4622 = t439 * t4620 / 27.0_f64;
    let t4624 = 4.0_f64 / 45.0_f64 * t1423 * t1969;
    let t4626 = t2002 * t1431 / 45.0_f64;
    let t4628 = t1887 * t460 / 15.0_f64;
    let t4630 = t802 * t1542 / 30.0_f64;
    (t4619, t4620, t4622, t4624, t4626, t4628, t4630)
}
