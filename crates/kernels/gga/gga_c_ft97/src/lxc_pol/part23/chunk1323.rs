//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1323/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1323<F: Float>(t1882: F, t31946: F, t5299: F, t6260: F, t31832: F, t31736: F, t8392: F, t31732: F, t31808: F, t112969: F, t11593: F, t1255: F, t2749: F, t28496: F, t2862: F, t2874: F, t29259: F, t29307: F, t31627: F, t31777: F, t319: F, t3746: F, t4129: F, t4162: F, t4246: F, t446: F, t7131: F, t840: F, t882: F, t98823: F) -> (F, F) {
    let t126112 = t1882 * t31946;
    let t126118 = t6260 * t5299;
    let t126123 = t1882 * t31832;
    let t126125 = t8392 * t31736;
    let t126131 = t8392 * t31732;
    let t126138 = t8392 * t31808;
    let t126156 = -4.0 / 9.0 * t126112 + 2.0 / 3.0 * t446 * t2862 * t882 * t31627 + 2.0 / 3.0 * t446 * t2862 * t319 * t126118 + t126123 / 9.0 - 2.0 / 27.0 * t126125 + 2.0 / 3.0 * t446 * t840 * t2749 * t31777 - 2.0 / 27.0 * t126131 + 4.0 / 3.0 * t446 * t2862 * t1255 * t28496 + 4.0 / 81.0 * t98823 + 4.0 / 9.0 * t126138 + 4.0 / 3.0 * t446 * t2862 * t7131 * t4162 + 2.0 / 3.0 * t446 * t840 * t4246 * t29307 - 2.0 / 3.0 * t446 * t840 * t7131 * t4129 - t112969 - 4.0 / 9.0 * t11593 * t2874 * t29259 * t3746;
    (t126118, t126156)
}
