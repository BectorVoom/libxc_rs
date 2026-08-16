//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1258/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1258(t41642: f64, t41646: f64, t41651: f64, t41655: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t10568: f64, t690: f64) -> (f64, f64) {
    let t41677 = 0.10685e0_f64 * t41642 + 0.14246666666666666667e0_f64 * t41646 + 0.42739999999999999999e0_f64 * t41651 + t41655 - 0.47488888888888888888e-1_f64 * t41656 - 0.31659259259259259258e-1_f64 * t41658 + 0.26382716049382716049e-1_f64 * t41660 + 0.23744444444444444444e-1_f64 * t41662 - 0.52765432098765432099e-1_f64 * t41669 - 0.17808333333333333333e-1_f64 * t41673 + 0.94977777777777777776e-1_f64 * t41675;
    let t41678 = t690 * t10568;
    (t41677, t41678)
}
