//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2425/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2425<F: Float>(t48760: F, t49256: F, t49259: F, t49262: F, t49268: F, t49271: F, t49273: F, t49276: F, t49530: F, t49563: F, t49567: F, t49572: F, t49575: F, t49585: F) -> F {
    let t49588 = t48760 + t49530 + t49563 - t49567 - t49572 + t49256 + t49259 + t49262 - t49575 + t49268 + t49271 + t49273 + t49276 + t49585;
    t49588
}
