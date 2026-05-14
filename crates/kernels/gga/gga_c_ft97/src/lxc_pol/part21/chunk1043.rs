//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1043/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1043<F: Float>(t38953: F, t5944: F, t2101: F, t5929: F, t5875: F, t8232: F, t5866: F, t1378: F, t9132: F, t582: F, t5935: F, t5842: F, t604: F, t23455: F, t50249: F, t23571: F, t50235: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95676 = t38953 * t5944;
    let t95696 = t2101 * t5929;
    let t95738 = t8232 * t5875;
    let t95740 = t8232 * t5866;
    let t95751 = t9132 * t1378;
    let t95767 = t582 * t5929;
    let t95789 = t2101 * t5935;
    let t95813 = t604 * t5842;
    let t95837 = t50249 * t23455;
    let t95842 = t50235 * t23571;
    (t95676, t95696, t95738, t95740, t95751, t95767, t95789, t95813, t95837, t95842)
}
