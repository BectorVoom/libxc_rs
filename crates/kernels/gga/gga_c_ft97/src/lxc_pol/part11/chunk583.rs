//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 583/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk583<F: Float>(t419: F, t8122: F, t423: F, t7745: F, t420: F, t1675: F, t67: F, t9: F) -> (F, F, F, F, F) {
    let t8123 = t419 * t8122;
    let t8125 = t423 * t7745;
    let t8126 = t420 * t8125;
    let t8127 = t419 * t8126;
    let t8130 = t9 * t67 * t1675;
    (t8123, t8125, t8126, t8127, t8130)
}
