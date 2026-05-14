//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 890/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk890<F: Float>(t113: F, t7921: F, t6086: F, t6085: F, t7605: F, t6093: F, t2294: F, t2583: F, t2582: F, t6063: F, t2155: F, t2207: F, t2208: F, t2837: F, t2559: F, t7494: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7922 = t7921 * t113;
    let t7923 = t6086 * t7922;
    let t7925 = 0.11643651550782197811e-1 * t6085 * t7923;
    let t7926 = t6086 * t7605;
    let t7928 = 0.34930954652346593434e-1 * t6093 * t7926;
    let t7937 = t2294 * t2583;
    let t7939 = 0.23115257973478049502e0 * t2582 * t7937;
    let t7949 = t6063 * t7605;
    let t7951 = 0.19514881078765566037e-1 * t2155 * t7949;
    let t7961 = t2207 * t2837 * t2208;
    let t7968 = 0.12805040077930161442e0 * t7494 * t2559;
    (t7922, t7923, t7925, t7926, t7928, t7937, t7939, t7949, t7951, t7961, t7968)
}
