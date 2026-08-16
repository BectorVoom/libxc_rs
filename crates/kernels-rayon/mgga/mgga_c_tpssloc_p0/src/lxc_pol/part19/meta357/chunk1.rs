//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1295/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295(t2859: f64, t2884: f64, t302: f64, t41642: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41845: f64) -> (f64, f64) {
    let t42154 = t302 / t2884 / t2859;
    let t42172 = 0.13892666666666666667e1_f64 * t41831 + 0.166712e1_f64 * t41833 - 0.125034e1_f64 * t41836 - 0.104195e0_f64 * t41839 + 0.250068e1_f64 * t41842 + 0.62517e0_f64 * t41845 + 0.309885e1_f64 * t41642 - 0.13772666666666666666e1_f64 * t41656 - 0.91817777777777777776e0_f64 * t41658 + 0.76514814814814814814e0_f64 * t41660 + 0.68863333333333333332e0_f64 * t41662 - 0.15302962962962962963e1_f64 * t41669 - 0.516475e0_f64 * t41673 + 0.27545333333333333333e1_f64 * t41675;
    (t42154, t42172)
}
