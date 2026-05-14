//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1103/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1103<F: Float>(t157: F, t40266: F, t27257: F, t8392: F, t27015: F, t50249: F, t604: F, t6615: F, t1378: F, t9224: F, t26868: F, t1391: F, t9114: F, t526: F, t1882: F, t26872: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106555 = t40266 * t157;
    let t106561 = 2.0 / 27.0 * t8392 * t27257;
    let t106565 = t50249 * t27015;
    let t106573 = t604 * t6615;
    let t106588 = t9224 * t1378;
    let t106600 = 2.0 / 27.0 * t8392 * t26868;
    let t106619 = t9114 * t1391;
    let t106623 = t526 * t1378;
    let t106639 = 2.0 / 27.0 * t1882 * t26872;
    (t106555, t106561, t106565, t106573, t106588, t106600, t106619, t106623, t106639)
}
