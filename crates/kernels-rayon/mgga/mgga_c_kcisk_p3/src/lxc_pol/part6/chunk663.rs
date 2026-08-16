//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 663/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk663(t2029: f64, t9234: f64, t1994: f64, t2648: f64, t5344: f64, t6949: f64, t6951: f64, t6959: f64, t7648: f64, t795: f64, t8482: f64, t8487: f64, t8668: f64, t8675: f64, t8679: f64, t8860: f64, t8863: f64, t9155: f64, t9163: f64) -> (f64, f64) {
    let t9235 = t9234 * t2029;
    let t9240 = -t5344 - 0.23214722222222222222e-2_f64 * t8482 + 0.15476481481481481481e-2_f64 * t8487 + 0.17411041666666666666e-2_f64 * t8668 + t9155 * t795 + 0.15476481481481481481e-2_f64 * t6949 - 0.23214722222222222222e-2_f64 * t6951 + 0.34822083333333333332e-2_f64 * t8675 + 0.92858888888888888886e-2_f64 * t8679 + 0.15476481481481481481e-2_f64 * t6959 + 0.193e0_f64 * t1994 * t9163 + 0.17024129629629629629e-1_f64 * t8860 - 0.92858888888888888886e-2_f64 * t8863 - 0.193e0_f64 * t1994 * t9235 - 0.386e0_f64 * t7648 * t2648;
    (t9235, t9240)
}
