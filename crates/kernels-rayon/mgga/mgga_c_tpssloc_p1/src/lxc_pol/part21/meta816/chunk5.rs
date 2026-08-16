//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2878/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878(t60106: f64, t60120: f64, t60133: f64, t60147: f64, t901: f64, t42444: f64, t48140: f64, t55716: f64, t43317: f64, t41656: f64, t41658: f64, t41675: f64, t41684: f64, t41863: f64, t41870: f64, t41872: f64, t47738: f64, t48103: f64, t48116: f64, t59655: f64, t60091: f64) -> (f64, f64, f64, f64, f64) {
    let t60149 = t60106 + t60120 + t60133 + t60147;
    let t60150 = t901 * t60149;
    let t60153 = t48140 * t42444 * t55716;
    let t60156 = t48140 * t43317 * t55716;
    let t60158 = 0.12077e1_f64 * t47738 + 0.49057777777777777779e0_f64 * t48103 - 0.13418888888888888889e0_f64 * t41656 - 0.8945925925925925926e-1_f64 * t41658 + 0.26837777777777777778e0_f64 * t41675 + 0.62621481481481481482e0_f64 * t41684 + 0.49057777777777777778e0_f64 * t41863 - 0.91983333333333333333e-1_f64 * t41870 - 0.30661111111111111111e-1_f64 * t41872 + 0.49057777777777777777e-1_f64 * t48116 - 0.198684e1_f64 * t60091 - 0.72462e1_f64 * t59655 + 0.16504875e0_f64 * t60150 + 0.66228e0_f64 * t60153 - 0.14717333333333333333e0_f64 * t60156;
    (t60149, t60150, t60153, t60156, t60158)
}
