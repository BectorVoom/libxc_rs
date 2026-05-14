//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 833/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk833<F: Float>(t11863: F, t1643: F, t1651: F, t1853: F, t1866: F, t1871: F, t1901: F, t1904: F, t1922: F, t358: F, t37298: F, t39093: F, t39095: F, t39097: F, t39099: F, t39101: F, t39107: F, t446: F, t447: F, t499: F, t7973: F, t8544: F) -> (F,) {
    let t39116 = -4.0 / 9.0 * t446 * t447 * t499 * t7973 - 4.0 / 9.0 * t446 * t1866 * t1922 * t1643 - 2.0 / 3.0 * t446 * t447 * t1922 * t1651 + 112.0 / 243.0 * t39093 + 8.0 / 9.0 * t39095 + 16.0 / 9.0 * t39097 - 16.0 / 27.0 * t39099 + 40.0 / 243.0 * t39101 + 8.0 * t446 * t1871 * t499 * t8544 + 8.0 / 3.0 * t1901 * t39107 * t1853 * t358 * t1904 - 8.0 / 3.0 * t1901 * t11863 * t37298;
    (t39116,)
}
