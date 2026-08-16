//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2240/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2240(t17617: f64, t26870: f64, t3682: f64, t8172: f64, t29020: f64, t3704: f64, t29086: f64, t3678: f64, t16733: f64, t16738: f64, t16742: f64, t17515: f64, t29047: f64, t29048: f64, t97174: f64, t97267: f64, t97269: f64, t97272: f64) -> f64 {
    let t104953 = 0.57165357490759649296e-3_f64 * t26870 * t17617;
    let t104963 = t8172 * t3682;
    let t104968 = 0.30488190661738479624e-2_f64 * t29020 * t3704;
    let t104972 = 0.57165357490759649296e-3_f64 * t29086 * t3678;
    let t104973 = -t104953 - t29047 * t29048 * t16738 / 72.0_f64 - t29047 * t29048 * t16742 / 144.0_f64 - t29047 * t29048 * t16733 / 48.0_f64 + t104963 / 162.0_f64 - 0.19055119163586549765e-3_f64 * t97267 + 0.28582678745379824648e-3_f64 * t97269 + t97272 - t104968 + 0.57165357490759649296e-3_f64 * t97174 * t17515 - t104972;
    t104973
}
