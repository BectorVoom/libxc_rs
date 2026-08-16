//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 930/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk930(t1882: f64, t8399: f64, t8402: f64, t104: f64, t38061: f64, t89: f64, t8207: f64, t8392: f64, t8220: f64, t487: f64, t7800: f64, t11810: f64, t12020: f64, t1559: f64, t1580: f64, t1588: f64, t1643: f64, t1755: f64, t1876: f64, t1901: f64, t1902: f64, t1903: f64, t3187: f64, t3193: f64, t38937: f64, t38947: f64, t8217: f64, t8510: f64, t8518: f64) -> f64 {
    let t39311 = t1882 * t8399;
    let t39313 = t1882 * t8402;
    let t39317 = 280.0_f64 / 243.0_f64 * t89 * t38061 * t104;
    let t39323 = t8392 * t8207;
    let t39329 = t8392 * t8220;
    let t39345 = t487 * t7800;
    let t39350 = 4.0_f64 / 3.0_f64 * t39311 + 4.0_f64 / 9.0_f64 * t39313 + t39317 - 4.0_f64 / 3.0_f64 * t1901 * t8217 * t1903 * t1580 * t1588 + 8.0_f64 / 9.0_f64 * t39323 + 4.0_f64 / 9.0_f64 * t1901 * t3193 * t8510 * t1643 + 8.0_f64 / 9.0_f64 * t39329 - 8.0_f64 * t1901 * t11810 * t487 * t1755 * t1876 - 4.0_f64 / 3.0_f64 * t1901 * t1902 * t3187 * t1559 * t1755 - 16.0_f64 / 9.0_f64 * t1901 * t8518 * t12020 * t38947 - 16.0_f64 / 9.0_f64 * t1901 * t3193 * t39345 * t38937;
    t39350
}
