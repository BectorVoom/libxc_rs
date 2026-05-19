//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1223/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1223<F: Float>(t3187: F, t37816: F, t38131: F, t40131: F, t40156: F, t40158: F, t40162: F, t41711: F, t43586: F, t43588: F, t43592: F, t43594: F, t43597: F) -> F {
    let t43599 = t37816 * t3187;
    let t43601 = F::cast_from(0.93149212406257582492e-1_f64) * t40131 - t41711 - F::cast_from(0.10975748638225852664e-1_f64) * t43586 - F::cast_from(0.43341108700271342816e-1_f64) * t43588 - t40156 + t40158 - F::cast_from(0.13972381860938637374e0_f64) * t40162 + F::cast_from(0.21831846657716620896e-2_f64) * t43592 - t38131 - F::cast_from(0.87327386630866483584e-2_f64) * t43594 - F::cast_from(0.43663693315433241792e-2_f64) * t43597 - F::cast_from(0.38415120233790484326e0_f64) * t43599;
    t43601
}
