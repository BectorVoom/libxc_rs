//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1203/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1203<F: Float>(t17376: F, t17425: F, t17483: F, t17706: F, t1506: F, t1628: F, t6220: F, t2128: F, t4481: F, t4314: F, t6188: F, t1615: F, t11780: F, t11799: F, t11811: F, t11815: F, t12890: F, t15800: F, t15804: F, t15810: F, t15813: F, t15817: F, t15821: F, t15824: F, t15826: F, t15830: F, t15832: F, t15836: F, t15840: F, t4414: F, t6208: F) -> (F, F, F, F, F) {
    let t17708 = t17376 + t17425 + t17483 + t17706;
    let t17709 = t1506 * t17708;
    let t17710 = t6220 * t1628;
    let t17713 = t2128 * t4481;
    let t17730 = t6188 * t4314;
    let t17731 = t17730 * t1615;
    let t17738 = 0.38691203703703703703e-3 * t15800 - 0.23214722222222222222e-2 * t15804 + 0.15476481481481481481e-2 * t11780 - 0.23214722222222222222e-2 * t11799 + 0.23214722222222222222e-2 * t15810 - 0.17411041666666666666e-2 * t15813 + 0.46429444444444444443e-2 * t15817 - 0.92858888888888888886e-2 * t15821 + 0.92858888888888888886e-2 * t15824 - 0.25794135802469135802e-3 * t15826 + 0.77382407407407407407e-3 * t15830 - 0.23214722222222222222e-2 * t15832 + 0.12897067901234567901e-2 * t15836 - 0.11607361111111111111e-2 * t15840 + 0.178089025e-1 * t4414 * t17731 + 0.178089025e-1 * t12890 * t6208 + 0.77382407407407407407e-3 * t11811 + 0.12897067901234567901e-2 * t11815;
    (t17709, t17710, t17713, t17731, t17738)
}
