//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1236/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1236<F: Float>(t6553: F, t7488: F, t1880: F, t1492: F, t1902: F, t1496: F, t6581: F, t1484: F, t236: F) -> (F, F, F, F, F) {
    let t7489 = t6553 * t7488;
    let t7490 = t1880 * t7489;
    let t7492 = t1492 * t1902;
    let t7494 = t6581 * t1496;
    let t7496 = t236 * t1484;
    (t7489, t7490, t7492, t7494, t7496)
}
