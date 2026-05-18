//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 841/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk841<F: Float>(t1268: F, t6842: F, t1240: F, t1857: F, t430: F, t5003: F, t5345: F, t6558: F, t6561: F, t6564: F, t6616: F, t6622: F, t6627: F, t6631: F, t6738: F, t6835: F) -> (F, F) {
    let t6843 = t6842 * t1268;
    let t6855 = F::new(0.11607361111111111111e-2) * t6558 - F::new(0.34822083333333333332e-2) * t6561 + F::new(0.23214722222222222222e-2) * t6564 - F::new(0.17411041666666666666e-2) * t6616 - F::new(0.66725e-1) * t1240 * t6843 - F::new(0.13345e0) * t5345 * t1857 + F::new(0.15476481481481481481e-2) * t5003 + F::new(0.66725e-1) * t1240 * t6738 + t6835 * t430 - F::new(0.23214722222222222222e-2) * t6622 + F::new(0.15476481481481481481e-2) * t6627 - F::new(0.23214722222222222222e-2) * t6631;
    (t6843, t6855)
}
