//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1084/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1084<F: Float>(t3262: F, t3472: F, t42919: F, t11523: F, t12033: F, t1115: F, t2530: F, t3270: F, t3579: F, t11338: F, t12567: F, t3465: F, t43717: F, t1044: F, t1149: F, t12365: F, t12964: F, t44147: F, t44150: F, t44152: F, t44155: F, t44158: F, t44161: F, t44165: F, t44168: F, t44519: F, t860: F, t9782: F) -> (F, F, F, F, F, F) {
    let t44524 = 15.0 / 8.0 * t3262 * t3472 * t42919;
    let t44526 = t11523 * t12033 / 2.0;
    let t44530 = t3579 * t3270 * t1115 * t2530 / 2.0;
    let t44532 = t12567 * t11338 / 4.0;
    let t44535 = 3.0 / 4.0 * t3262 * t3465 * t43717;
    let t44536 = 2.0 * t1044 * t12365 + t1149 * t9782 + t12964 * t860 + t44147 - t44150 + t44152 + t44155 + t44158 - t44161 + t44165 + t44168 + t44519 - t44524 - t44526 + t44530 - t44532 - t44535;
    (t44524, t44526, t44530, t44532, t44535, t44536)
}
