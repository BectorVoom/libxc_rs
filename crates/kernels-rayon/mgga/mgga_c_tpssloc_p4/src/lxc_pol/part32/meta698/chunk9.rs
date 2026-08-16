//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2184/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2184(t22633: f64, t26338: f64, t90566: f64, t22751: f64, t28213: f64, t28210: f64, t28233: f64, t6883: f64, t1323: f64, t16439: f64, t19804: f64, t2006: f64, t22656: f64, t28107: f64, t28187: f64, t3882: f64, t568: f64, t6361: f64, t6461: f64, t6955: f64, t7750: f64, t81284: f64, t90702: f64, t90708: f64) -> (f64, f64, f64) {
    let t97527 = t22633 * t90566 * t26338;
    let t97529 = t22751 * t28213;
    let t97537 = t22751 * t28210;
    let t97548 = t6883 * t28233;
    let t97552 = t90702 + 0.38381794893125283518e-1_f64 * t97537 - t3882 * t28187 - t22656 * t6461 + t19804 * t2006 * t568 + t6361 * t6955 * t568 + 0.16449340668482264365e-1_f64 * t81284 + t90708 + t1323 * t28107 * t568 - 0.38381794893125283518e-1_f64 * t97548 - 2.0_f64 * t16439 * t7750;
    (t97527, t97529, t97552)
}
