//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 741/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk741<F: Float>(t4114: F, t1592: F, t3725: F, t3729: F, t3731: F, t3736: F, t3740: F, t3957: F, t4112: F, t4117: F, t4127: F, t4315: F, t4390: F, t626: F) -> (F, F) {
    let t4399 = F::new(0.38691203703703703703e-3) * t4114;
    let t4402 = F::new(0.66725e-1) * t1592 * t4315 + t4390 * t626 + F::new(0.11607361111111111111e-2) * t3725 - F::new(0.23214722222222222222e-2) * t3729 + F::new(0.15476481481481481481e-2) * t3731 - F::new(0.34822083333333333332e-2) * t3736 + F::new(0.23214722222222222222e-2) * t3740 - F::new(0.17411041666666666666e-2) * t3957 + F::new(0.17411041666666666666e-2) * t4112 - t4399 + F::new(0.23214722222222222222e-2) * t4117 + F::new(0.34822083333333333332e-2) * t4127;
    (t4399, t4402)
}
