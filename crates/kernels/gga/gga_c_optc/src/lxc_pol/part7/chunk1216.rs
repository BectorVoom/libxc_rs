//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1216/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1216<F: Float>(t26261: F, t26309: F, t26311: F, t26314: F, t26319: F, t26324: F, t26326: F, t26328: F, t26330: F, t26332: F, t26339: F, t26343: F, t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F) -> (F, F) {
    let t26808 = 0.17757530864197530864e0 * t26261;
    let t26818 = 0.4566222222222222222e-1 * t26309 - 0.9132444444444444444e-1 * t26311 + t26808 + 0.22831111111111111111e-1 * t26314 + 0.13698666666666666667e0 * t26319 - 0.4566222222222222222e-1 * t26324 - 0.45662222222222222221e-1 * t26326 - 0.3044148148148148148e-1 * t26328 + 0.9132444444444444444e-1 * t26330 + 0.71030123456790123454e-1 * t26332 - 0.50735802469135802467e-1 * t26339 - 0.17123333333333333333e-1 * t26343;
    let t26833 = 0.26382716049382716049e-1 * t26252 + 0.23744444444444444444e0 * t26258 - 0.11872222222222222222e0 * t26278 + 0.14246666666666666667e0 * t26280 - 0.42739999999999999999e0 * t26284 + 0.42739999999999999999e0 * t26289 - 0.35616666666666666666e-1 * t26293 + 0.4274e0 * t26296 - 0.6411e0 * t26300 + 0.10685e0 * t26304 - 0.14246666666666666667e0 * t26306;
    (t26818, t26833)
}
