//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1223/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1223(t3187: f64, t37816: f64, t38131: f64, t40131: f64, t40156: f64, t40158: f64, t40162: f64, t41711: f64, t43586: f64, t43588: f64, t43592: f64, t43594: f64, t43597: f64) -> f64 {
    let t43599 = t37816 * t3187;
    let t43601 = 0.93149212406257582492e-1_f64 * t40131 - t41711 - 0.10975748638225852664e-1_f64 * t43586 - 0.43341108700271342816e-1_f64 * t43588 - t40156 + t40158 - 0.13972381860938637374e0_f64 * t40162 + 0.21831846657716620896e-2_f64 * t43592 - t38131 - 0.87327386630866483584e-2_f64 * t43594 - 0.43663693315433241792e-2_f64 * t43597 - 0.38415120233790484326e0_f64 * t43599;
    t43601
}
