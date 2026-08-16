//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1212/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1212<F: Float>(t18532: F, t19115: F, t19118: F, t19121: F, t19124: F, t19128: F, t19130: F, t19132: F, t19137: F, t19142: F, t19144: F, t19151: F, t19153: F, t19155: F, t19157: F, t19162: F, t19301: F, t19304: F, t19307: F) -> F {
    let t20248 = -F::cast_from(0.25794135802469135802e-3_f64) * t18532 - F::cast_from(0.17411041666666666666e-2_f64) * t19115 - F::cast_from(0.23214722222222222222e-2_f64) * t19118 - F::cast_from(0.77382407407407407407e-3_f64) * t19121 - F::cast_from(0.61905925925925925925e-2_f64) * t19124 + F::cast_from(0.61905925925925925925e-2_f64) * t19128 - F::cast_from(0.15476481481481481481e-2_f64) * t19130 - F::cast_from(0.23214722222222222222e-2_f64) * t19132 - F::cast_from(0.23214722222222222222e-2_f64) * t19137 - F::cast_from(0.46429444444444444444e-2_f64) * t19142 + F::cast_from(0.15476481481481481481e-2_f64) * t19144 + F::cast_from(0.30952962962962962962e-2_f64) * t19151 - F::cast_from(0.23214722222222222222e-2_f64) * t19153 - F::cast_from(0.23214722222222222221e-2_f64) * t19155 + F::cast_from(0.15476481481481481481e-2_f64) * t19157 - F::cast_from(0.19345601851851851852e-2_f64) * t19162 - F::cast_from(0.23214722222222222221e-2_f64) * t19301 - F::cast_from(0.34822083333333333332e-2_f64) * t19304 - F::cast_from(0.17411041666666666666e-2_f64) * t19307;
    t20248
}
