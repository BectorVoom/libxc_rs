//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1042/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1042<F: Float>(t1882: F, t24927: F, t38953: F, t6362: F, t25215: F, t24898: F, t56456: F, t25309: F, t24923: F, t25379: F, t24905: F, t8392: F, t24910: F, t24957: F, t25295: F, t25206: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t99656 = t1882 * t24927;
    let t99665 = t38953 * t6362;
    let t99670 = t1882 * t25215;
    let t99672 = t56456 * t24898;
    let t99676 = t1882 * t25309;
    let t99678 = t1882 * t24923;
    let t99693 = t1882 * t25379;
    let t99703 = t8392 * t24905;
    let t99706 = t8392 * t24910;
    let t99712 = t1882 * t24957;
    let t99717 = t1882 * t25295;
    let t99719 = t1882 * t25206;
    (t99656, t99665, t99670, t99672, t99676, t99678, t99693, t99703, t99706, t99712, t99717, t99719)
}
