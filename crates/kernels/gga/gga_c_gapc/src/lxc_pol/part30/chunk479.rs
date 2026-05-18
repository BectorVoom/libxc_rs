//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 479/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk479<F: Float>(t2674: F, t285: F, t191: F, t2254: F, t332: F, t330: F, t197: F, t617: F, t936: F, t1854: F, t942: F, t1882: F, t320: F) -> (F, F, F, F, F, F) {
    let t2675 = t2674 * t285;
    let t2676 = t2675 * t191;
    let t2677 = t332 * t2254;
    let t2678 = t330 * t2677;
    let t2679 = t197 * t2678;
    let t2682 = t617 * t936;
    let t2685 = t1854 * t942;
    let t2690 = t320 * t1882;
    (t2675, t2676, t2679, t2682, t2685, t2690)
}
