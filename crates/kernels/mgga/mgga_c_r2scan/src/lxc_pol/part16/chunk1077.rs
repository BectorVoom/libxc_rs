//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1077/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1077<F: Float>(t3309: F, t9327: F, t2147: F, t29936: F, t3332: F, t11683: F, t26088: F, t10760: F, t29946: F, t6535: F, t3187: F, t37816: F, t38131: F, t40131: F, t40156: F, t40158: F, t40162: F, t41711: F, t43586: F) -> (F,) {
    let t43588 = t9327 * t3309;
    let t43592 = t2147 * t3332 * t29936;
    let t43594 = t26088 * t11683;
    let t43597 = t6535 * t10760 * t29946;
    let t43599 = t37816 * t3187;
    let t43601 = 0.93149212406257582492e-1 * t40131 - t41711 - 0.10975748638225852664e-1 * t43586 - 0.43341108700271342816e-1 * t43588 - t40156 + t40158 - 0.13972381860938637374e0 * t40162 + 0.21831846657716620896e-2 * t43592 - t38131 - 0.87327386630866483584e-2 * t43594 - 0.43663693315433241792e-2 * t43597 - 0.38415120233790484326e0 * t43599;
    (t43601,)
}
