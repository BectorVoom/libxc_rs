//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1008/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1008(t597: f64, t9114: f64, t12982: f64, t13212: f64, t144: f64, t1651: f64, t1901: f64, t1986: f64, t2179: f64, t2180: f64, t2185: f64, t2190: f64, t39646: f64, t39660: f64, t41084: f64, t41093: f64, t446: f64, t558: f64, t574: f64, t9099: f64, t9117: f64, t9123: f64, t9144: f64, t9349: f64, t9354: f64, t9419: f64, t9439: f64, t9440: f64) -> f64 {
    let t41107 = t9114 * t597;
    let t41117 = 8.0_f64 * t446 * t574 * t9439 * t9440 * t558 + 8.0_f64 / 3.0_f64 * t41084 + 8.0_f64 * t446 * t2185 * t2179 * t1986 * t2180 + t41093 - 4.0_f64 / 3.0_f64 * t1901 * t9144 * t1651 * t2190 + 8.0_f64 / 9.0_f64 * t1901 * t13212 * t39660 + 8.0_f64 / 3.0_f64 * t446 * t144 * t39646 + 8.0_f64 / 9.0_f64 * t1901 * t12982 * t9123 + 8.0_f64 / 9.0_f64 * t1901 * t41107 * t9117 + 4.0_f64 / 3.0_f64 * t1901 * t9419 * t9349 + 4.0_f64 / 3.0_f64 * t1901 * t9099 * t9354;
    t41117
}
