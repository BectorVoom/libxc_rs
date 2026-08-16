//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 884/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk884<F: Float>(t38075: F, t38110: F, t1685: F, t77: F, t7983: F, t37453: F, t534: F, t1608: F, t1611: F, t533: F, t1593: F, t37481: F) -> (F, F, F, F, F) {
    let t38111 = t38075 + t38110;
    let t38117 = t7983 * t77 * t1685;
    let t38120 = t534 * t37453;
    let t38129 = t1608 * t77 * t1611 * t533;
    let t38146 = t1608 * t77 * t37481 * t1593;
    (t38111, t38117, t38120, t38129, t38146)
}
