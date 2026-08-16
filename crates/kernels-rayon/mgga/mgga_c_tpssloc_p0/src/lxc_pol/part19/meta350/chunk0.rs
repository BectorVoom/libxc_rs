//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1272/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1272(t2904: f64, t41733: f64, t951: f64, t959: f64, t41654: f64, t41642: f64, t41646: f64, t41651: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64) -> (f64, f64) {
    let t41737 = 0.35089341735807877242e1_f64 * t959 * t2904 * t41733 * t951;
    let t41741 = 0.96141975308641975307e-1_f64 * t41654;
    let t41749 = 0.55625000000000000001e-1_f64 * t41642 + 0.74166666666666666668e-1_f64 * t41646 + 0.22249999999999999999e0_f64 * t41651 + t41741 - 0.24722222222222222222e-1_f64 * t41656 - 0.16481481481481481482e-1_f64 * t41658 + 0.13734567901234567901e-1_f64 * t41660 + 0.12361111111111111111e-1_f64 * t41662 - 0.27469135802469135803e-1_f64 * t41669 - 0.92708333333333333333e-2_f64 * t41673 + 0.49444444444444444445e-1_f64 * t41675;
    (t41737, t41749)
}
