//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1196/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1196<F: Float>(t4835: F, t4846: F, t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t23769: F, t23770: F, t30189: F, t30270: F, t49378: F, t49381: F, t56978: F, t56981: F, t56984: F, t56988: F, t56991: F, t56994: F) -> (F, F, F, F) {
    let t58109 = t4835 * t4835;
    let t58115 = t4846 * t4846;
    let t58132 = -0.19384444444444444445e4 * t39411 - 0.12922962962962962963e4 * t39413 + 0.38768888888888888889e4 * t39418 + 0.19384444444444444445e4 * t49240 - 0.58153333333333333333e4 * t49242 - 0.12586666666666666667e4 * t49271 + 0.20977777777777777778e3 * t49273 + 17446.0 * t56966 - 0.4846111111111111111e4 * t56969 - 0.10488888888888888889e3 * t56972 - 0.20977777777777777778e3 * t56975;
    let t58143 = -26169.0 * t56978 + 0.58153333333333333332e4 * t56981 - 0.19384444444444444444e4 * t56984 - 2832.0 * t56988 + 0.62933333333333333332e3 * t56991 + 0.94399999999999999998e3 * t56994 + 0.93234567901234567903e3 * t30189 + t23769 + t23770 + 0.932345679012345679e2 * t49378 + 0.20977777777777777778e3 * t49381 + 0.30153580246913580247e4 * t30270;
    (t58109, t58115, t58132, t58143)
}
