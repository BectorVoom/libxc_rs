//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1690/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1690(t3678: f64, t5323: f64, t1235: f64, t1238: f64, t12800: f64, t12976: f64, t17280: f64, t17283: f64, t17290: f64, t17296: f64, t1791: f64, t1808: f64, t3644: f64, t3663: f64, t3667: f64, t5320: f64, t5327: f64, t5391: f64) -> f64 {
    let t17298 = 0.15244095330869239812e-2_f64 * t5323 * t3678;
    let t17299 = -0.21437009059034868486e-3_f64 * t12976 * t1791 - 0.42874018118069736972e-3_f64 * t3667 * t5320 + 0.15244095330869239812e-2_f64 * t5391 * t3644 - 0.14291339372689912324e-3_f64 * t12800 * t1808 - 0.21437009059034868486e-3_f64 * t1235 * t17280 + 0.22866142996303859718e-2_f64 * t17283 * t1238 + 0.11433071498151929859e-2_f64 * t5323 * t3663 - 0.42874018118069736972e-3_f64 * t17290 * t1238 - 0.21437009059034868486e-3_f64 * t5327 * t3663 - t17296 + t17298;
    t17299
}
