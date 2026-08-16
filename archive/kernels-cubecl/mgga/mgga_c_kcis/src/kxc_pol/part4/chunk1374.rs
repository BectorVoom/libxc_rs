//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1374/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1374<F: Float>(t11780: F, t11799: F, t11811: F, t11815: F, t12890: F, t15800: F, t15804: F, t15810: F, t15813: F, t15817: F, t15821: F, t15824: F, t15826: F, t15830: F, t15832: F, t15836: F, t15840: F, t17731: F, t4414: F, t6208: F) -> F {
    let t17738 = F::cast_from(0.38691203703703703703e-3_f64) * t15800 - F::cast_from(0.23214722222222222222e-2_f64) * t15804 + F::cast_from(0.15476481481481481481e-2_f64) * t11780 - F::cast_from(0.23214722222222222222e-2_f64) * t11799 + F::cast_from(0.23214722222222222222e-2_f64) * t15810 - F::cast_from(0.17411041666666666666e-2_f64) * t15813 + F::cast_from(0.46429444444444444443e-2_f64) * t15817 - F::cast_from(0.92858888888888888886e-2_f64) * t15821 + F::cast_from(0.92858888888888888886e-2_f64) * t15824 - F::cast_from(0.25794135802469135802e-3_f64) * t15826 + F::cast_from(0.77382407407407407407e-3_f64) * t15830 - F::cast_from(0.23214722222222222222e-2_f64) * t15832 + F::cast_from(0.12897067901234567901e-2_f64) * t15836 - F::cast_from(0.11607361111111111111e-2_f64) * t15840 + F::cast_from(0.178089025e-1_f64) * t4414 * t17731 + F::cast_from(0.178089025e-1_f64) * t12890 * t6208 + F::cast_from(0.77382407407407407407e-3_f64) * t11811 + F::cast_from(0.12897067901234567901e-2_f64) * t11815;
    t17738
}
