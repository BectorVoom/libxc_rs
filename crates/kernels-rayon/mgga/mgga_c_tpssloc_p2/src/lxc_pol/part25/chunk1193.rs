//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1193/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1193(t80825: f64, t80847: f64, t80807: f64, t80810: f64, t80814: f64, t80817: f64, t80821: f64, t80828: f64, t80831: f64, t80833: f64, t80837: f64, t80843: f64, t80850: f64, t80857: f64, t80859: f64, t80861: f64, t80863: f64, t80867: f64, t80870: f64, t80872: f64) -> f64 {
    let t84514 = 0.2034786907144675699e0_f64 * t80825;
    let t84520 = 455.0_f64 / 648.0_f64 * t80847;
    let t84529 = 0.20186378047070195427e-3_f64 * t80807 + t80810 / 768.0_f64 + 0.12111826828242117256e-2_f64 * t80814 + t80817 / 32.0_f64 - 7.0_f64 / 48.0_f64 * t80821 - t84514 - 7.0_f64 / 8.0_f64 * t80828 - t80831 / 2.0_f64 + t80833 / 64.0_f64 + 0.60559134141210586279e-3_f64 * t80837 - 0.84782787797694820791e-2_f64 * t80843 - t84520 - t80850 / 64.0_f64 - 0.24223653656484234512e-2_f64 * t80857 - 35.0_f64 / 96.0_f64 * t80859 - 5.0_f64 / 32.0_f64 * t80861 + 5.0_f64 / 64.0_f64 * t80863 - 119.0_f64 / 288.0_f64 * t80867 + 7.0_f64 / 48.0_f64 * t80870 + 7.0_f64 / 96.0_f64 * t80872;
    t84529
}
