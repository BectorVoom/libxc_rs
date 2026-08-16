//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 853/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk853(t1165: f64, t1338: f64, t1799: f64, t3493: f64, t5801: f64, t6234: f64, t6309: f64, t6323: f64, t5909: f64, t6245: f64, t5913: f64, t5916: f64, t6249: f64, t6251: f64, t6253: f64) -> (f64, f64, f64) {
    let t6409 = 2.0_f64 * t1165 * t6323 + 2.0_f64 * t1338 * t5801 + 2.0_f64 * t1799 * t3493 + 2.0_f64 * t1799 * t6234 + t6309;
    let t6413 = t5909 * t6245;
    let t6419 = -t5913 - t6249 / 24.0_f64 - t6251 / 768.0_f64 - t5916 - t6253 / 192.0_f64;
    (t6409, t6413, t6419)
}
