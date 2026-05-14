//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 555/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk555<F: Float>(t1557: F, t422: F, t7765: F, t420: F, t419: F, t1527: F, t7789: F, t1725: F, t1744: F, t173: F, t1743: F, t1736: F, t7800: F, t3088: F, t7807: F, t424: F, t626: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8088 = t422 * t1557;
    let t8089 = t8088 * t7765;
    let t8090 = t420 * t8089;
    let t8091 = t419 * t8090;
    let t8093 = t1527 * t7789;
    let t8094 = t419 * t8093;
    let t8096 = t1725 * t1744;
    let t8098 = t173 * t1743;
    let t8099 = t419 * t8098;
    let t8101 = t1736 * t7800;
    let t8102 = t8101 * t7765;
    let t8103 = t420 * t8102;
    let t8104 = t419 * t8103;
    let t8106 = t3088 * t7807;
    let t8107 = t419 * t8106;
    let t8109 = t626 * t424;
    (t8089, t8090, t8091, t8093, t8094, t8096, t8098, t8099, t8102, t8103, t8104, t8106, t8107, t8109)
}
