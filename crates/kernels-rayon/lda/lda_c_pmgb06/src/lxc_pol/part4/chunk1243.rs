//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1243/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1243(t12456: f64, t12460: f64, t12462: f64, t12465: f64, t1462: f64, t1465: f64, t1981: f64, t764: f64, t1963: f64, t5220: f64, t2591: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16372 = 8.0_f64 / 405.0_f64 * t12456;
    let t16373 = 8.0_f64 / 405.0_f64 * t12460;
    let t16374 = 8.0_f64 / 405.0_f64 * t12462;
    let t16375 = 128.0_f64 / 405.0_f64 * t12465;
    let t16379 = 4.0_f64 / 27.0_f64 * t1981 * t1462 * t1465 * t764;
    let t16380 = t5220 * t1963;
    let t16381 = 8.0_f64 / 135.0_f64 * t16380;
    let t16382 = t2591 * t607;
    (t16372, t16373, t16374, t16375, t16379, t16381, t16382)
}
