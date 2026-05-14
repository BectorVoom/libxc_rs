//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 733/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk733<F: Float>(t1882: F, t3467: F, t12606: F, t144: F, t1053: F, t1986: F, t2185: F, t605: F, t12306: F, t12308: F, t12310: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12315: F, t12881: F) -> (F, F, F, F) {
    let t13084 = 2.0 / 27.0 * t1882 * t3467;
    let t13085 = t144 * t12606;
    let t13088 = t1053 * t1986;
    let t13090 = t2185 * t605 * t13088;
    let t13100 = 2.0 / 9.0 * t12306;
    let t13101 = 4.0 / 9.0 * t12308;
    let t13102 = 4.0 / 27.0 * t12310;
    let t13104 = t12881 / 2.0 + t12285 / 3.0 + 2.0 / 9.0 * t12290 - 10.0 / 27.0 * t12293 - 8.0 / 9.0 * t12296 + t12300 / 3.0 + 4.0 / 3.0 * t12304 - t13100 - t13101 + t13102 - 2.0 / 3.0 * t12315;
    (t13084, t13085, t13090, t13104)
}
