//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 663/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk663<F: Float>(t2029: F, t9234: F, t1994: F, t2648: F, t5344: F, t6949: F, t6951: F, t6959: F, t7648: F, t795: F, t8482: F, t8487: F, t8668: F, t8675: F, t8679: F, t8860: F, t8863: F, t9155: F, t9163: F) -> (F, F) {
    let t9235 = t9234 * t2029;
    let t9240 = -t5344 - F::new(0.23214722222222222222e-2) * t8482 + F::new(0.15476481481481481481e-2) * t8487 + F::new(0.17411041666666666666e-2) * t8668 + t9155 * t795 + F::new(0.15476481481481481481e-2) * t6949 - F::new(0.23214722222222222222e-2) * t6951 + F::new(0.34822083333333333332e-2) * t8675 + F::new(0.92858888888888888886e-2) * t8679 + F::new(0.15476481481481481481e-2) * t6959 + F::new(0.193e0) * t1994 * t9163 + F::new(0.17024129629629629629e-1) * t8860 - F::new(0.92858888888888888886e-2) * t8863 - F::new(0.193e0) * t1994 * t9235 - F::new(0.386e0) * t7648 * t2648;
    (t9235, t9240)
}
