//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 744/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk744<F: Float>(t2061: F, t7902: F, t494: F, t938: F, t113: F, t6086: F, t6085: F, t7605: F, t6093: F, t2294: F, t2583: F, t2582: F, t6063: F, t2155: F, t2207: F, t2208: F, t2837: F) -> (F, F, F, F, F, F, F) {
    let t7904 = 0.1350520664e0 * t2061 * t7902;
    let t7921 = t938 * t494;
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
    (t7904, t7922, t7925, t7928, t7939, t7951, t7961)
}
