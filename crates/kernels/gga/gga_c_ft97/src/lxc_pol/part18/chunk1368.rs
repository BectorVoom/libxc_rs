//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1368/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1368<F: Float>(t26868: F, t8392: F, t1391: F, t9114: F, t1378: F, t526: F, t1882: F, t26872: F, t105353: F, t105388: F, t105518: F, t1060: F, t12277: F, t12590: F, t12703: F, t12714: F, t13142: F, t13166: F, t13212: F, t1557: F, t1647: F, t1901: F, t2185: F, t2212: F, t2221: F, t23527: F, t23548: F, t27006: F, t27034: F, t3188: F, t446: F, t50558: F, t574: F, t5869: F, t5968: F, t9144: F, t95643: F, t95649: F) -> (F,) {
    let t106600 = 2.0 / 27.0 * t8392 * t26868;
    let t106619 = t9114 * t1391;
    let t106623 = t526 * t1378;
    let t106639 = 2.0 / 27.0 * t1882 * t26872;
    let t106647 = -t106600 + 4.0 / 9.0 * t1901 * t12703 * t105518 + 2.0 / 3.0 * t1901 * t50558 * t23548 * t13166 + 2.0 / 3.0 * t446 * t574 * t12277 * t5869 + 2.0 / 81.0 * t95643 + t95649 / 27.0 + 4.0 / 27.0 * t1901 * t12714 * t5968 * t1557 * t3188 + 4.0 / 27.0 * t1901 * t106619 * t12590 - 4.0 / 3.0 * t1901 * t106623 * t13142 - 2.0 / 9.0 * t1901 * t2221 * t27006 * t1647 + 4.0 / 3.0 * t446 * t2185 * t1060 * t23527 + 2.0 / 27.0 * t1901 * t13212 * t105353 + t106639 - 4.0 / 9.0 * t1901 * t12703 * t105388 - 2.0 / 9.0 * t1901 * t9144 * t27034 * t2212;
    (t106647,)
}
