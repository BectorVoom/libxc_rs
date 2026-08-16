//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 597/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk597(t958: f64, t962: f64, t2004: f64, t332: f64, t917: f64, t921: f64, t215: f64, t334: f64, t671: f64, t333: f64, t2465: f64, t970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2660 = t958 * t962;
    let t2662 = t2004 * t332;
    let t2665 = t917 * t921;
    let t2668 = t215 * t671 * t334;
    let t2670 = t333 * t2668 / 432.0_f64;
    let t2671 = t970 * t2465;
    (t2660, t2662, t2665, t2668, t2670, t2671)
}
