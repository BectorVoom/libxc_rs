//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 800/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk800<F: Float>(t2582: F, t7937: F, t6063: F, t7605: F, t2155: F, t2207: F, t2208: F, t2837: F, t2559: F, t7494: F, t2526: F, t277: F) -> (F, F, F, F, F) {
    let t7939 = F::new(0.23115257973478049502e0) * t2582 * t7937;
    let t7949 = t6063 * t7605;
    let t7951 = F::new(0.19514881078765566037e-1) * t2155 * t7949;
    let t7961 = t2207 * t2837 * t2208;
    let t7968 = F::new(0.12805040077930161442e0) * t7494 * t2559;
    let t7977 = t277 * t2526;
    (t7939, t7951, t7961, t7968, t7977)
}
