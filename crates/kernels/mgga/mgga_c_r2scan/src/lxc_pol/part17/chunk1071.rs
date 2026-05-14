//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1071/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1071<F: Float>(t37700: F, t37707: F, t39579: F, t41464: F, t41466: F, t43178: F, t43181: F, t43183: F, t43185: F, t43188: F, t43191: F, t39635: F, t39637: F, t39642: F, t39672: F, t41474: F, t41475: F, t41479: F, t43195: F, t43200: F, t43203: F, t43205: F, t43209: F) -> (F, F) {
    let t44268 = -0.51220160311720645768e0 * t39579 + 0.11708928647259339623e0 * t37700 - 0.45022119329691164871e0 * t37707 + 0.52396431978519890152e-1 * t43178 - 0.13099107994629972538e-1 * t43181 - 0.87327386630866483588e-2 * t43183 - 0.26198215989259945076e-1 * t43185 + t41464 + 0.13099107994629972538e-1 * t43188 - t41466 - 0.13972381860938637374e0 * t43191;
    let t44278 = 0.26198215989259945076e-1 * t43195 + t41474 + t41475 - 0.50853567541651708904e1 * t39635 - 0.65854491829355115985e-1 * t39637 - t41479 + 0.23417857294518679244e0 * t39642 + 0.26198215989259945076e-1 * t43200 - 0.17465477326173296718e-1 * t43203 - 0.51220160311720645768e0 * t39672 - 0.26198215989259945076e-1 * t43205 + 0.13099107994629972538e-1 * t43209;
    (t44268, t44278)
}
