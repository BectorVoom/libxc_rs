//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 934/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk934<F: Float>(t1882: F, t20108: F, t20117: F, t376: F, t89: F, t20113: F, t358: F, t20142: F, t20131: F, t20114: F, t375: F, t20099: F) -> (F, F, F, F, F, F, F) {
    let t73301 = t1882 * t20108;
    let t73343 = t89 * t376 * t20117;
    let t73345 = t20113 * t358;
    let t73358 = t1882 * t20142;
    let t73405 = t1882 * t20131;
    let t73439 = t89 * t375 * t20114;
    let t73442 = t89 * t375 * t20099;
    (t73301, t73343, t73345, t73358, t73405, t73439, t73442)
}
