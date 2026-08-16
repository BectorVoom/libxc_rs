//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3758/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3758(t17763: f64, t5378: f64, t3568: f64, t6587: f64, t12800: f64, t12866: f64, t17344: f64, t17351: f64, t17354: f64, t17401: f64, t17514: f64, t17724: f64, t1808: f64, t21272: f64, t247: f64, t3620: f64, t3719: f64, t58863: f64, t59173: f64, t59176: f64, t59179: f64, t59182: f64, t59185: f64, t6673: f64, t71300: f64) -> (f64, f64) {
    let t71598 = t17763 * t5378;
    let t71606 = t6587 * t3568;
    let t71624 = -0.3811023832717309953e-3_f64 * t71598 + 0.23818898954483187207e-3_f64 * t12800 * t6673 + 0.80454947579587654563e-2_f64 * t21272 * t3620 + 0.15244095330869239812e-2_f64 * t58863 * t1808 - 0.12862205435420921092e-2_f64 * t17344 * t247 * t3719 * t71606 - 0.28582678745379824648e-3_f64 * t59173 - 0.57165357490759649296e-3_f64 * t59176 + 0.28582678745379824648e-3_f64 * t59179 + 0.7622047665434619906e-3_f64 * t59182 - 0.85748036236139473944e-3_f64 * t17401 * t17724 + 0.19055119163586549765e-3_f64 * t59185 + 0.28582678745379824648e-3_f64 * t17351 * t71300 * t17354 + 0.28582678745379824648e-3_f64 * t12866 * t71300 * t17514;
    (t71606, t71624)
}
