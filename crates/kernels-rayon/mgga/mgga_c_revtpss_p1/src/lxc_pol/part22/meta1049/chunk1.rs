//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3687/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3687(t1230: f64, t21271: f64, t1266: f64, t12800: f64, t17763: f64, t1808: f64, t21242: f64, t21272: f64, t3640: f64, t3644: f64, t5397: f64, t57187: f64, t6683: f64, t69698: f64, t69700: f64, t69710: f64, t69719: f64, t69721: f64) -> f64 {
    let t69723 = t1230 * t21271;
    let t69728 = -0.3811023832717309953e-3_f64 * t69698 - 0.95275595817932748827e-4_f64 * t69700 - 0.96545937095505185476e-2_f64 * t21272 * t3644 - 0.28582678745379824648e-3_f64 * t12800 * t6683 - 0.28582678745379824648e-3_f64 * t57187 * t1808 - 0.57165357490759649296e-3_f64 * t17763 * t5397 + 0.30488190661738479624e-2_f64 * t69710 * t1266 + 0.15244095330869239812e-2_f64 * t21242 * t3640 + 0.30488190661738479624e-2_f64 * t21242 * t3644 + 0.57165357490759649296e-3_f64 * t69719 - 0.60976381323476959248e-2_f64 * t69721 - 0.96545937095505185476e-2_f64 * t69723 * t1266 - 0.48272968547752592738e-2_f64 * t21272 * t3640;
    t69728
}
