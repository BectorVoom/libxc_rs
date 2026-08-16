//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1486/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1486(t12116: f64, t4891: f64, t3172: f64, t4874: f64, t3127: f64, t4802: f64, t1063: f64, t4807: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t11737: f64, t11745: f64, t3106: f64, t4803: f64, t4808: f64, t4896: f64) -> f64 {
    let t15758 = t12116 * t4891;
    let t15769 = t3172 * t4874;
    let t15771 = 0.19055119163586549765e-3_f64 * t3127 * t15769;
    let t15772 = t3172 * t4802;
    let t15774 = 0.3811023832717309953e-3_f64 * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15779 = 0.85748036236139473944e-3_f64 * t15758 * t4896 + 0.95275595817932748827e-4_f64 * t11723 + 0.15879265969655458138e-3_f64 * t11728 + 11.0_f64 / 324.0_f64 * t11730 + t11732 / 81.0_f64 + t11737 + 0.30488190661738479624e-2_f64 * t3106 * t4803 - 0.2540682555144873302e-2_f64 * t3106 * t4808 - t15771 - t15774 + 0.31758531939310916276e-3_f64 * t15776 - 0.19055119163586549765e-3_f64 * t11745;
    t15779
}
