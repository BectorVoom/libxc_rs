//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 694/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk694<F: Float>(t1882: F, t2614: F, t2581: F, t2469: F, t2526: F, t242: F, t2542: F, t761: F, t766: F, t192: F, t7514: F, t265: F, t9708: F, t10090: F, t10094: F, t10123: F, t10126: F, t10128: F, t10131: F, t10134: F, t10137: F, t10140: F, t10143: F, t1901: F, t446: F) -> (F, F, F, F, F, F, F, F) {
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10150 = t2469 * t2526;
    let t10151 = t242 * t10150;
    let t10153 = t2542 * t761;
    let t10154 = t10153 * t766;
    let t10155 = t242 * t10154;
    let t10157 = t192 * t7514;
    let t10159 = t10157 * t265 * t9708;
    let t10162 = -2.0 / 9.0 * t10090 + t1901 * t10094 / 3.0 - t446 * t10123 / 3.0 + t10126 / 9.0 + 2.0 / 27.0 * t10128 - t446 * t10131 / 9.0 - 4.0 / 27.0 * t10134 - t446 * t10137 / 3.0 + 2.0 / 9.0 * t10140 + 2.0 / 3.0 * t446 * t10143 - 2.0 / 9.0 * t10146 - 2.0 / 3.0 * t10148 - t446 * t10151 - t446 * t10155 - 2.0 * t446 * t10159;
    (t10150, t10151, t10153, t10154, t10155, t10157, t10159, t10162)
}
