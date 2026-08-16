//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 424/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk424(t1465: f64, t947: f64, t242: f64, t1407: f64, t970: f64, t1461: f64, t923: f64, t925: f64, t946: f64, t964: f64, t967: f64) -> (f64, f64, f64) {
    let t1466 = t947 * t1465;
    let t1467 = t242 * t1466;
    let t1470 = t970 * t1407;
    let t1471 = t242 * t1470;
    let t1474 = t923 + t925 * t1461 / 288.0_f64 + t946 * t1467 / 3072.0_f64 + t964 + t967 * t1471 / 4608.0_f64;
    (t1467, t1471, t1474)
}
