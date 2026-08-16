//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2244/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2244(t29019: f64, t5273: f64, t20973: f64, t7624: f64, t1785: f64, t29082: f64, t104727: f64, t104739: f64, t104742: f64, t1252: f64, t1266: f64, t1808: f64, t21200: f64, t21267: f64, t26852: f64, t29037: f64, t29040: f64, t3670: f64, t5386: f64, t5397: f64, t6631: f64, t6673: f64, t6683: f64, t97206: f64) -> f64 {
    let t112252 = t5273 * t29019;
    let t112258 = t7624 * t20973;
    let t112260 = t1785 * t29082;
    let t112278 = -0.45732285992607719436e-2_f64 * t112252 * t1252 + 0.85748036236139473944e-3_f64 * t97206 * t6631 - 0.38110238327173099531e-3_f64 * t104742 - 0.19055119163586549765e-3_f64 * t112258 + 0.30488190661738479624e-2_f64 * t112260 * t1266 + 0.17149607247227894789e-2_f64 * t29040 * t21200 - 0.91464571985215438872e-2_f64 * t3670 * t29082 * t5386 + 0.47637797908966374413e-3_f64 * t26852 * t6673 - 0.57165357490759649296e-3_f64 * t26852 * t6683 + 0.30488190661738479624e-2_f64 * t104739 * t1808 - 0.57165357490759649296e-3_f64 * t29037 * t5397 - 0.25724410870841842183e-2_f64 * t104727 * t21267;
    t112278
}
