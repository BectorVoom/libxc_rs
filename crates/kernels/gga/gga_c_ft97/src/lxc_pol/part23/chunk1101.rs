//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1101/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1101<F: Float>(t2567: F, t6187: F, t38953: F, t6163: F, t6081: F, t8232: F, t6090: F, t6101: F, t6156: F, t24737: F, t53891: F, t458: F, t5995: F, t24181: F, t683: F, t2404: F, t6008: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97928 = t2567 * t6187;
    let t97952 = t38953 * t6163;
    let t97964 = t8232 * t6081;
    let t97966 = t8232 * t6090;
    let t98061 = t8232 * t6101;
    let t98078 = t8232 * t6156;
    let t98123 = t53891 * t24737;
    let t98152 = t5995 * t458;
    let t98168 = t683 * t24181;
    let t98195 = t2404 * t6008;
    (t97928, t97952, t97964, t97966, t98061, t98078, t98123, t98152, t98168, t98195)
}
