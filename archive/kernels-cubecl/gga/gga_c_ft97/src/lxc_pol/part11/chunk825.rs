//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 825/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk825<F: Float>(t81: F, t8633: F, t2258: F, t1711: F, t25: F, t371: F, t142: F, t2112: F, t358: F, t10915: F, t240: F, t2917: F) -> (F, F, F, F, F, F, F, F) {
    let t15568 = t8633 * t81;
    let t15575 = t2258 * t81;
    let t15810 = t1711 * t25;
    let t15811 = t371 * t15810;
    let t16633 = t8633 * t142;
    let t16640 = t2258 * t142;
    let t17338 = t2112 * t358;
    let t17687 = t10915 * t240;
    let t17694 = t2917 * t240;
    (t15568, t15575, t15811, t16633, t16640, t17338, t17687, t17694)
}
