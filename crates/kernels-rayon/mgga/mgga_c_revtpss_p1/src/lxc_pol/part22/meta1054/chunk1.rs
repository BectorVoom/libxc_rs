//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3726/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3726(t1250: f64, t12787: f64, t16733: f64, t17353: f64, t17420: f64, t17625: f64, t17693: f64, t17705: f64, t17713: f64, t17729: f64, t17730: f64, t17760: f64, t20265: f64, t20292: f64, t21014: f64, t44225: f64, t57421: f64, t57428: f64, t59411: f64, t70639: f64, t70647: f64, t70664: f64, t70667: f64, t70672: f64) -> f64 {
    let t70675 = 0.25724410870841842183e-2_f64 * t70639 * t17713 - 0.47637797908966374414e-3_f64 * t17729 * t12787 * t20265 * t17730 + 0.5081365110289746604e-2_f64 * t70647 * t17760 - 0.17149607247227894789e-2_f64 * t17693 * t17353 * t1250 * t16733 - 0.67751534803863288054e-3_f64 * t57421 - 0.91464571985215438872e-2_f64 * t21014 * t17420 + 0.1270341277572436651e-2_f64 * t17729 * t44225 * t20292 * t17730 + 0.85748036236139473944e-3_f64 * t59411 * t17625 - 0.57165357490759649296e-3_f64 * t70664 + 0.28582678745379824648e-3_f64 * t70667 - 0.45732285992607719436e-2_f64 * t21014 * t17705 + 0.3811023832717309953e-3_f64 * t70672 + t57428 / 54.0_f64;
    t70675
}
