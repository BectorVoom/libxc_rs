//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 573/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk573<F: Float>(t3565: F, t528: F, t1645: F, t2792: F, t3556: F, t1: F, t3516: F, t106: F, t192: F) -> (F, F, F, F, F, F) {
    let t11389 = t528 * t3565;
    let t11392 = t1645 * t2792;
    let t11395 = t528 * t3556;
    let t11400 = t3516 * t1;
    let t11401 = t11400 * t106;
    let t11402 = t11401 * t192;
    (t11389, t11392, t11395, t11400, t11401, t11402)
}
