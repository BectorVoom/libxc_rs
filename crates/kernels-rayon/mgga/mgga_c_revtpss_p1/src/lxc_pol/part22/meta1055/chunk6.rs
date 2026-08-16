//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3736/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736(t12987: f64, t5390: f64, t1250: f64, t12910: f64, t17347: f64, t17448: f64, t17635: f64, t17753: f64, t17754: f64, t3584: f64, t3720: f64, t5402: f64, t57100: f64, t57726: f64, t57735: f64, t57743: f64, t57746: f64, t57749: f64, t57770: f64, t57773: f64, t57780: f64, t6688: f64, t70890: f64) -> f64 {
    let t70959 = t12987 * t5390;
    let t70978 = 0.85748036236139473944e-3_f64 * t12910 * t3720 * t6688 * t1250 * t3584 + 0.13719685797782315831e-1_f64 * t70959 * t17347 + 0.21437009059034868486e-3_f64 * t17753 * t3720 * t70890 * t17754 - t57726 / 243.0_f64 - 0.11433071498151929859e-2_f64 * t57735 - 0.57165357490759649296e-3_f64 * t57100 * t5402 - 0.57165357490759649296e-3_f64 * t17448 * t17635 - t57743 / 108.0_f64 - t57746 / 216.0_f64 - t57749 / 72.0_f64 - 0.3811023832717309953e-3_f64 * t57770 + 0.19055119163586549765e-3_f64 * t57773 - 0.57165357490759649296e-3_f64 * t57780;
    t70978
}
