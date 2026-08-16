//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 924/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk924(t1859: f64, t8232: f64, t1882: f64, t8579: f64, t1786: f64, t1852: f64, t11863: f64, t1643: f64, t1651: f64, t1853: f64, t1866: f64, t1871: f64, t1901: f64, t1904: f64, t1922: f64, t358: f64, t37298: f64, t39093: f64, t39095: f64, t39097: f64, t446: f64, t447: f64, t499: f64, t7973: f64, t8544: f64) -> f64 {
    let t39099 = t8232 * t1859;
    let t39101 = t1882 * t8579;
    let t39107 = t1786 * t1852;
    let t39116 = -4.0_f64 / 9.0_f64 * t446 * t447 * t499 * t7973 - 4.0_f64 / 9.0_f64 * t446 * t1866 * t1922 * t1643 - 2.0_f64 / 3.0_f64 * t446 * t447 * t1922 * t1651 + 112.0_f64 / 243.0_f64 * t39093 + 8.0_f64 / 9.0_f64 * t39095 + 16.0_f64 / 9.0_f64 * t39097 - 16.0_f64 / 27.0_f64 * t39099 + 40.0_f64 / 243.0_f64 * t39101 + 8.0_f64 * t446 * t1871 * t499 * t8544 + 8.0_f64 / 3.0_f64 * t1901 * t39107 * t1853 * t358 * t1904 - 8.0_f64 / 3.0_f64 * t1901 * t11863 * t37298;
    t39116
}
