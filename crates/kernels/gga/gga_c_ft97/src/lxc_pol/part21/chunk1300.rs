//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1300/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1300<F: Float>(t1882: F, t30491: F, t30479: F, t30447: F, t8392: F, t11593: F, t119270: F, t119477: F, t119488: F, t119577: F, t119581: F, t12703: F, t13208: F, t144: F, t16951: F, t16955: F, t17002: F, t17189: F, t1901: F, t23443: F, t23470: F, t23548: F, t23892: F, t30400: F, t446: F, t4822: F, t4839: F, t574: F, t5842: F, t9099: F, t9144: F, t95541: F) -> (F,) {
    let t120383 = t1882 * t30491;
    let t120385 = t1882 * t30479;
    let t120400 = t8392 * t30447;
    let t120411 = -2.0 / 9.0 * t1901 * t9144 * t95541 * t4822 - 2.0 / 9.0 * t1901 * t9144 * t23548 * t17189 + 4.0 / 9.0 * t1901 * t12703 * t119577 + 2.0 / 9.0 * t1901 * t9144 * t23892 * t16955 + 2.0 / 3.0 * t1901 * t13208 * t119581 - 2.0 / 9.0 * t120383 + 2.0 / 9.0 * t120385 - 2.0 / 3.0 * t446 * t144 * t119488 - t446 * t574 * t4839 * t5842 / 3.0 + 4.0 / 9.0 * t11593 * t23470 * t17002 - 2.0 / 9.0 * t1901 * t23443 * t16951 + 2.0 / 27.0 * t120400 + 2.0 / 3.0 * t446 * t144 * t119477 + 4.0 / 3.0 * t446 * t144 * t119270 + 2.0 / 9.0 * t1901 * t9099 * t30400;
    (t120411,)
}
