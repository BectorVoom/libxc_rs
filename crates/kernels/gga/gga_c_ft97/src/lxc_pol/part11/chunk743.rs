//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 743/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk743<F: Float>(t120: F, t1595: F, t528: F, t167: F, t9132: F, t582: F, t605: F, t2097: F, t157: F, t9224: F, t160: F, t7763: F, t7800: F, t1570: F, t586: F, t1557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12488 = t1595 * t528 * t120;
    let t12703 = t9132 * t167;
    let t12709 = t582 * t605;
    let t12714 = t2097 * t605;
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12746 = t160 * t7800;
    let t12791 = t586 * t1570;
    let t12796 = t586 * t1557;
    (t12488, t12703, t12709, t12714, t12723, t12724, t12746, t12791, t12796)
}
