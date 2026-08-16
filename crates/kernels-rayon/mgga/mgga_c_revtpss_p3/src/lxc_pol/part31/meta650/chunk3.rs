//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2147/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2147(t20054: f64, t7132: f64, t20050: f64, t100092: f64, t100097: f64, t100117: f64, t20066: f64, t25577: f64, t27493: f64, t6323: f64, t6327: f64, t93611: f64, t93618: f64, t93622: f64) -> f64 {
    let t106934 = t7132 * t20054;
    let t106938 = t7132 * t20050;
    let t106943 = 0.85748036236139473944e-3_f64 * t27493 * t20066 - 0.15244095330869239812e-2_f64 * t25577 * t6323 + 0.19055119163586549765e-3_f64 * t106934 - 0.2540682555144873302e-2_f64 * t25577 * t6327 + 0.31758531939310916275e-3_f64 * t106938 - t100092 + t100097 + t93611 + 0.50813651102897466041e-3_f64 * t93618 - 0.95275595817932748827e-4_f64 * t93622 + 0.38110238327173099531e-3_f64 * t100117;
    t106943
}
