//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1281/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281(t41642: f64, t41646: f64, t41651: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41904: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41690: f64, t41695: f64, t41699: f64, t41703: f64, t41707: f64, t41711: f64, t41713: f64, t41717: f64) -> (f64, f64) {
    let t41912 = 2.0_f64 * t41642 + 8.0_f64 / 3.0_f64 * t41646 + 8.0_f64 * t41651 + t41904 - 8.0_f64 / 9.0_f64 * t41656 - 16.0_f64 / 27.0_f64 * t41658 + 40.0_f64 / 81.0_f64 * t41660 + 4.0_f64 / 9.0_f64 * t41662 - 80.0_f64 / 81.0_f64 * t41669 - t41673 / 3.0_f64 + 16.0_f64 / 9.0_f64 * t41675;
    let t41925 = -16.0_f64 / 9.0_f64 * t41678 + 8.0_f64 / 9.0_f64 * t41680 + 8.0_f64 / 3.0_f64 * t41682 + 112.0_f64 / 81.0_f64 * t41684 + 40.0_f64 / 9.0_f64 * t41690 - 20.0_f64 / 9.0_f64 * t41695 - 8.0_f64 * t41699 - 2.0_f64 / 3.0_f64 * t41703 - 8.0_f64 / 9.0_f64 * t41707 + 8.0_f64 * t41711 - 8.0_f64 / 3.0_f64 * t41713 - 12.0_f64 * t41717;
    (t41912, t41925)
}
