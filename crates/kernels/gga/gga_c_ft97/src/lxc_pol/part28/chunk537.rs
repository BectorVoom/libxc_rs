//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 537/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk537<F: Float>(t1384: F, t2178: F, t5968: F, t604: F, t5944: F, t8392: F, t1378: F, t582: F) -> (F, F, F, F) {
    let t23455 = t2178 * t1384;
    let t23463 = t604 * t5968;
    let t23468 = t8392 * t5944;
    let t23470 = t582 * t1378;
    (t23455, t23463, t23468, t23470)
}
