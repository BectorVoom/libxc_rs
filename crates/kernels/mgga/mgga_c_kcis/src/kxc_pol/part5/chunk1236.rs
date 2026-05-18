//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1236/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1236<F: Float>(t10473: F, t11151: F, t11209: F, t14609: F, t15659: F, t15662: F, t15671: F, t19779: F, t19783: F, t19787: F, t19792: F, t19800: F, t19802: F, t19805: F, t19809: F, t19813: F, t19817: F, t19819: F, t20550: F, t430: F, t6738: F) -> F {
    let t20706 = F::new(0.19345601851851851852e-2) * t19779 + F::new(0.23214722222222222222e-2) * t19783 - F::new(0.11607361111111111111e-2) * t19787 + F::new(0.890445125e-2) * t11151 * t6738 + F::new(0.34822083333333333332e-2) * t19792 + t15659 + t11209 + F::new(0.10317654320987654321e-2) * t10473 - t15662 - t15671 - F::new(0.51588271604938271603e-3) * t14609 + F::new(0.11607361111111111111e-2) * t19800 - F::new(0.15476481481481481481e-2) * t19802 - F::new(0.41270617283950617283e-2) * t19805 - F::new(0.11607361111111111111e-1) * t19809 + F::new(0.51588271604938271605e-2) * t19813 + F::new(0.77382407407407407408e-2) * t19817 + F::new(0.10317654320987654321e-2) * t19819 + t20550 * t430;
    t20706
}
