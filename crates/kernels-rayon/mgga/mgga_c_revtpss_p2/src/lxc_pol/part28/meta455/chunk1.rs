//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1726/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1726(t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t1261: f64, t1266: f64, t17721: f64, t17724: f64, t17729: f64, t17732: f64, t17736: f64, t17739: f64, t17744: f64, t17747: f64, t17750: f64, t17753: f64, t17756: f64, t17760: f64, t17763: f64, t3718: f64) -> f64 {
    let t17767 = 0.19055119163586549765e-3_f64 * t3647 * t5378;
    let t17769 = t247 * t3634 * t5056;
    let t17771 = 0.19055119163586549765e-3_f64 * t1261 * t17769;
    let t17772 = 0.31758531939310916276e-3_f64 * t17721 - 0.42874018118069736972e-3_f64 * t3718 * t17724 + 0.57165357490759649296e-3_f64 * t17729 * t17732 - 0.57165357490759649296e-3_f64 * t17736 * t17739 - 0.21437009059034868486e-3_f64 * t3718 * t17744 - 0.12862205435420921092e-2_f64 * t17747 * t17750 + 0.21437009059034868486e-3_f64 * t17753 * t17756 - 0.47637797908966374414e-3_f64 * t17729 * t17760 - 0.28582678745379824648e-3_f64 * t17763 * t1266 - t17767 - t17771;
    t17772
}
