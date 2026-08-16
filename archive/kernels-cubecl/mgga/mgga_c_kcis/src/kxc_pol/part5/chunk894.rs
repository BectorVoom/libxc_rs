//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 894/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk894<F: Float>(t1592: F, t2110: F, t4399: F, t5764: F, t5766: F, t6193: F, t626: F, t7102: F, t7106: F, t7109: F, t7196: F, t7199: F, t7205: F, t7208: F, t7260: F, t7264: F, t7490: F, t7510: F) -> F {
    let t7532 = F::cast_from(0.66725e-1_f64) * t1592 * t7510 + F::cast_from(0.15476481481481481481e-2_f64) * t5764 - F::cast_from(0.23214722222222222222e-2_f64) * t5766 - t4399 - F::cast_from(0.23214722222222222222e-2_f64) * t7102 + F::cast_from(0.17024129629629629629e-1_f64) * t7106 - F::cast_from(0.92858888888888888886e-2_f64) * t7109 + F::cast_from(0.17411041666666666666e-2_f64) * t7196 + F::cast_from(0.23214722222222222222e-2_f64) * t7199 + F::cast_from(0.11607361111111111111e-2_f64) * t7205 - F::cast_from(0.34822083333333333332e-2_f64) * t7208 - F::cast_from(0.17411041666666666666e-2_f64) * t7260 - F::cast_from(0.13345e0_f64) * t6193 * t2110 + t7490 * t626 - F::cast_from(0.61905925925925925925e-2_f64) * t7264;
    t7532
}
