//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2385/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385(t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41713: f64, t41904: f64, t47744: f64, t47748: f64, t47761: f64, t47765: f64, t47769: f64, t47777: f64, t47781: f64, t47785: f64, t47787: f64) -> f64 {
    let t48980 = 40.0_f64 / 9.0_f64 * t47744 + 8.0_f64 * t47748 - 4.0_f64 / 9.0_f64 * t41656 - 8.0_f64 / 27.0_f64 * t41658 + 10.0_f64 / 81.0_f64 * t41660 + t41662 / 9.0_f64 + 8.0_f64 / 9.0_f64 * t41675 - 4.0_f64 / 9.0_f64 * t41678 + 2.0_f64 / 3.0_f64 * t41682 + 28.0_f64 / 27.0_f64 * t41684 + 2.0_f64 * t47761 + 2.0_f64 * t47765 + 2.0_f64 / 3.0_f64 * t47769 + 2.0_f64 / 9.0_f64 * t41680 - 2.0_f64 / 3.0_f64 * t41713 + 4.0_f64 * t47777 + t41904 - 10.0_f64 / 9.0_f64 * t47781 - 6.0_f64 * t47785 + 28.0_f64 / 81.0_f64 * t47787;
    t48980
}
