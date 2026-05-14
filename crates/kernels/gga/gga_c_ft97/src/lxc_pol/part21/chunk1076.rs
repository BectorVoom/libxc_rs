//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1076/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1076<F: Float>(t1882: F, t26225: F, t26375: F, t8392: F, t1339: F, t8326: F, t26420: F, t26425: F, t26151: F, t488: F, t7750: F, t23339: F, t47667: F, t26340: F, t26276: F, t26242: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t103453 = 2.0 / 9.0 * t1882 * t26225;
    let t103459 = 4.0 / 3.0 * t8392 * t26375;
    let t103472 = t8326 * t1339;
    let t103486 = 2.0 / 9.0 * t1882 * t26420;
    let t103488 = 2.0 / 9.0 * t1882 * t26425;
    let t103490 = 4.0 / 9.0 * t1882 * t26151;
    let t103491 = t7750 * t488;
    let t103510 = t47667 * t23339;
    let t103515 = 2.0 / 9.0 * t1882 * t26340;
    let t103542 = 2.0 / 9.0 * t1882 * t26276;
    let t103550 = 2.0 / 9.0 * t1882 * t26242;
    (t103453, t103459, t103472, t103486, t103488, t103490, t103491, t103510, t103515, t103542, t103550)
}
