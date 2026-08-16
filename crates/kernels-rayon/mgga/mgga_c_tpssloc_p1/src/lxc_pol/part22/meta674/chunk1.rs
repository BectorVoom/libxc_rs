//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2232/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232(t10224: f64, t5828: f64, t973: f64, t42875: f64, t5817: f64, t17763: f64, t2960: f64, t18057: f64, t225: f64, t18059: f64, t1020: f64, t17960: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61597 = t973 * t10224 * t5828;
    let t61600 = t973 * t42875 * t5817;
    let t61602 = t2960 * t17763;
    let t61621 = t18057 * t225;
    let t61646 = t18059 * t225;
    let t61655 = t1020 * t248 * t3101 * t17960;
    (t61597, t61600, t61602, t61621, t61646, t61655)
}
