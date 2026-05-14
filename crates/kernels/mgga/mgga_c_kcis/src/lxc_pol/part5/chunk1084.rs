//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1084/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1084<F: Float>(t18532: F, t19115: F, t19118: F, t19121: F, t19124: F, t19128: F, t19130: F, t19132: F, t19137: F, t19142: F, t19144: F, t19151: F, t19153: F, t19155: F, t19157: F, t19162: F, t19301: F, t19304: F, t19307: F) -> (F,) {
    let t20248 = -0.25794135802469135802e-3 * t18532 - 0.17411041666666666666e-2 * t19115 - 0.23214722222222222222e-2 * t19118 - 0.77382407407407407407e-3 * t19121 - 0.61905925925925925925e-2 * t19124 + 0.61905925925925925925e-2 * t19128 - 0.15476481481481481481e-2 * t19130 - 0.23214722222222222222e-2 * t19132 - 0.23214722222222222222e-2 * t19137 - 0.46429444444444444444e-2 * t19142 + 0.15476481481481481481e-2 * t19144 + 0.30952962962962962962e-2 * t19151 - 0.23214722222222222222e-2 * t19153 - 0.23214722222222222221e-2 * t19155 + 0.15476481481481481481e-2 * t19157 - 0.19345601851851851852e-2 * t19162 - 0.23214722222222222221e-2 * t19301 - 0.34822083333333333332e-2 * t19304 - 0.17411041666666666666e-2 * t19307;
    (t20248,)
}
