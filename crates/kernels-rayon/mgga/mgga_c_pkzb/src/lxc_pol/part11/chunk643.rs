//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 643/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk643(t1096: f64, t1108: f64, t1916: f64, t1938: f64, t1955: f64, t1977: f64, t248: f64, t2796: f64, t2829: f64, t3521: f64, t3523: f64, t3527: f64, t3553: f64, t3556: f64, t3559: f64, t3565: f64, t3578: f64, t3581: f64, t3587: f64, t3592: f64, t3605: f64, t3608: f64, t695: f64, t714: f64) -> f64 {
    let t3611 = -0.310907e-1_f64 * t3559 * t248 + 2.0_f64 * t2796 * t1096 - 2.0_f64 * t1916 * t3565 + 1.0_f64 * t695 * t3578 + 0.32163958997385070134e2_f64 * t1938 * t3581 + t3521 - t3523 + t3527 - t3553 - t3556 - 0.19751673498613801407e-1_f64 * t3587 + 0.11696447245269292414e1_f64 * t2829 * t1108 - 0.11696447245269292414e1_f64 * t1955 * t3592 + 0.5848223622634646207e0_f64 * t714 * t3605 + 0.17315859105681463759e2_f64 * t1977 * t3608;
    t3611
}
