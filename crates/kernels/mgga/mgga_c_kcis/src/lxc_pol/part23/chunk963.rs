//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 963/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk963<F: Float>(t11780: F, t11799: F, t11811: F, t11815: F, t12890: F, t15800: F, t15804: F, t15810: F, t15813: F, t15817: F, t15821: F, t15824: F, t15826: F, t15830: F, t15832: F, t15836: F, t15840: F, t17731: F, t4414: F, t6208: F) -> F {
    let t17738 = F::new(0.38691203703703703703e-3) * t15800 - F::new(0.23214722222222222222e-2) * t15804 + F::new(0.15476481481481481481e-2) * t11780 - F::new(0.23214722222222222222e-2) * t11799 + F::new(0.23214722222222222222e-2) * t15810 - F::new(0.17411041666666666666e-2) * t15813 + F::new(0.46429444444444444443e-2) * t15817 - F::new(0.92858888888888888886e-2) * t15821 + F::new(0.92858888888888888886e-2) * t15824 - F::new(0.25794135802469135802e-3) * t15826 + F::new(0.77382407407407407407e-3) * t15830 - F::new(0.23214722222222222222e-2) * t15832 + F::new(0.12897067901234567901e-2) * t15836 - F::new(0.11607361111111111111e-2) * t15840 + F::new(0.178089025e-1) * t4414 * t17731 + F::new(0.178089025e-1) * t12890 * t6208 + F::new(0.77382407407407407407e-3) * t11811 + F::new(0.12897067901234567901e-2) * t11815;
    t17738
}
