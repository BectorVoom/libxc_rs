//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1486/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1486(t1921: f64, t8283: f64, t1455: f64, t8389: f64, t31619: f64, t571: f64, t2184: f64, t6951: f64, t2192: f64, t6936: f64, t117369: f64, t117374: f64, t117772: f64, t117774: f64, t1464: f64, t1914: f64, t2185: f64, t22571: f64, t31377: f64, t31583: f64, t5790: f64, t5808: f64, t8284: f64, t8373: f64) -> (f64, f64, f64) {
    let t117789 = 2.0_f64 * t8283 * t1921;
    let t117793 = 2.0_f64 * t1455 * t8389;
    let t118208 = t571 * t31619;
    let t118209 = t2184 * t6951;
    let t118213 = t6936 * t2192;
    let t118217 = t1464 * t31583 + 2.0_f64 * t1914 * t31377 + t2185 * t22571 + 2.0_f64 * t5790 * t8389 + 2.0_f64 * t5808 * t8373 + t6951 * t8284 + t117369 + t117374 + t117772 + t117774 + t118208 + t118209 + t118213;
    (t117789, t117793, t118217)
}
