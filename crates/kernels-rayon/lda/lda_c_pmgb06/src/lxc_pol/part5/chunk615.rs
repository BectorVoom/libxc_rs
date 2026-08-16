//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 615/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk615(t3955: f64, t2164: f64, t395: f64, t1461: f64, t842: f64, t1447: f64, t1995: f64, t1435: f64, t813: f64, t1423: f64, t1969: f64, t810: f64, t947: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4571 = 32.0_f64 * t3955;
    let t4579 = 0.2133002709687175_f64 * t395 * t2164;
    let t4588 = t1461 * t842;
    let t4593 = 4.0_f64 / 45.0_f64 * t1447 * t1995;
    let t4619 = t1435 * t813;
    let t4624 = 4.0_f64 / 45.0_f64 * t1423 * t1969;
    let t4635 = t947 * t810;
    (t4571, t4579, t4588, t4593, t4619, t4624, t4635)
}
