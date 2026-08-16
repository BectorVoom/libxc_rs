//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 178/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk178(t231: f64, t893: f64, t992: f64, t1093: f64, t1190: f64, t902: f64, t898: f64, t900: f64, t631: f64, t892: f64, t332: f64, t113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1263 = t231 * t893 * t992;
    let t1268 = 0.234754e0_f64 * t1190 - t902 - 0.14443083333333333333e0_f64 * t1093;
    let t1270 = t898 * t900 * t1268;
    let t1273 = t892 + t631 * t1263 / 6.0_f64 + t631 * t1270 / 2.0_f64;
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    (t1263, t1268, t1270, t1273, t1274, t1275)
}
