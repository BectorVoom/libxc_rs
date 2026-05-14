//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1027/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1027<F: Float>(t16588: F, t10379: F, t10410: F, t10417: F, t15919: F, t15924: F, t15933: F, t15939: F, t15942: F, t15945: F, t15949: F, t15951: F, t15953: F, t15955: F, t15958: F, t16572: F, t16578: F, t16583: F, t16586: F) -> (F,) {
    let t18222 = 0.15476481481481481481e-2 * t16588;
    let t18223 = -0.77382407407407407407e-2 * t15919 + 0.19345601851851851852e-2 * t15924 + 0.51588271604938271605e-2 * t15933 + 0.15476481481481481481e-2 * t10379 + 0.10317654320987654321e-2 * t15939 - 0.34822083333333333332e-2 * t15942 - 0.38691203703703703703e-3 * t15945 - 0.23214722222222222222e-2 * t15949 + 0.38691203703703703703e-3 * t15951 - 0.25794135802469135802e-3 * t15953 + 0.10317654320987654321e-2 * t15955 - 0.46429444444444444443e-2 * t15958 + 0.17411041666666666666e-2 * t16572 + 0.10317654320987654321e-2 * t10410 - 0.15476481481481481481e-2 * t10417 - 0.15476481481481481481e-2 * t16578 - 0.51588271604938271604e-3 * t16583 - 0.23214722222222222222e-2 * t16586 + t18222;
    (t18223,)
}
