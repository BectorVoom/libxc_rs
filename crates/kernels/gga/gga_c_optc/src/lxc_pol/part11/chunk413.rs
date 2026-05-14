//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 413/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk413<F: Float>(t136: F, t195: F, t222: F, t224: F, t864: F) -> (F, F, F, F) {
    let t2278 = t195 * t136;
    let t2280 = t222 * t2278 * t224;
    let t2281 = 0.20525e-2 * t2280;
    let t2284 = t136 * t864;
    (t2278, t2280, t2281, t2284)
}
