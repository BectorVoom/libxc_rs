//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1076/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1076<F: Float>(t39906: F, t39908: F, t41607: F, t41608: F, t41611: F, t41615: F, t43407: F, t43410: F, t43413: F, t43415: F, t43418: F, t43421: F, t38597: F, t39943: F, t39950: F, t41623: F, t41624: F, t43424: F, t43426: F, t43428: F, t43432: F, t43435: F, t43438: F, t43441: F) -> (F, F) {
    let t44380 = -t41607 - t41608 + 0.27013271597814698923e1 * t39906 - 0.13170898365871023197e0 * t39908 - t41611 - 0.17336443480108537126e0 * t43407 + 0.87327386630866483588e-2 * t43410 + 0.26198215989259945076e-1 * t43413 - 0.17465477326173296718e-1 * t43415 + t41615 + 0.46230515946956099003e0 * t43418 + 0.23115257973478049502e0 * t43421;
    let t44388 = -t41623 - 0.17336443480108537126e0 * t43424 - 0.17336443480108537126e0 * t43426 + t41624 + 0.12805040077930161442e0 * t43428 + t39943 - t38597 + t39950 + 0.54878743191129263322e-2 * t43432 - 0.46230515946956099003e0 * t43435 - 0.27738309568173659403e1 * t43438 + 0.93149212406257582492e-1 * t43441;
    (t44380, t44388)
}
