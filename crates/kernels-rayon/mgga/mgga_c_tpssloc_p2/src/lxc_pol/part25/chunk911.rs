//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 911/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk911(t11702: f64, t1213: f64, t3490: f64, t3523: f64, t1190: f64, t3030: f64, t3032: f64, t3505: f64, t10469: f64, t466: f64, t10471: f64, t1208: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11703 = t1213 * t11702;
    let t11705 = t3490 * t3523;
    let t11707 = t1190 * t3030;
    let t11708 = t11707 * t3032;
    let t11709 = t11708 * t3505;
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    let t11714 = t1208 * t1208;
    (t11703, t11705, t11707, t11708, t11709, t11712, t11713, t11714)
}
