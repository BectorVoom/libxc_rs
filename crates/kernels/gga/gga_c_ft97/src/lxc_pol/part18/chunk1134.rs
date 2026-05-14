//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1134/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1134<F: Float>(t24048: F, t376: F, t89: F, t24010: F, t8392: F, t1882: F, t23953: F, t23961: F, t5875: F, t8232: F, t5866: F, t23543: F, t23974: F, t23965: F, t23444: F, t1378: F, t9132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t95723 = t89 * t376 * t24048;
    let t95725 = t8392 * t24010;
    let t95730 = t1882 * t23953;
    let t95736 = t1882 * t23961;
    let t95738 = t8232 * t5875;
    let t95740 = t8232 * t5866;
    let t95742 = t1882 * t23543;
    let t95744 = t1882 * t23974;
    let t95747 = t1882 * t23965;
    let t95749 = t8392 * t23444;
    let t95751 = t9132 * t1378;
    (t95723, t95725, t95730, t95736, t95738, t95740, t95742, t95744, t95747, t95749, t95751)
}
