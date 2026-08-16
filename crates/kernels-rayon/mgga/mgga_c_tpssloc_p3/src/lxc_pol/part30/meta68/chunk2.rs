//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 455/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk455(t1323: f64, t562: f64, t541: f64, t801: f64, t119: f64, t1307: f64, t210: f64) -> (f64, f64, f64) {
    let t1324 = t1323 * t562;
    let t1327 = 7.0_f64 / 288.0_f64 * t801 * t541;
    let t1328 = t119 * t1307;
    let t1329 = t210 * t1328;
    (t1324, t1327, t1329)
}
