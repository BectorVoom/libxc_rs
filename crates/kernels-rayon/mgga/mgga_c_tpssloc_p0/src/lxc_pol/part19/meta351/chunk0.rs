//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1277/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1277(t2403: f64, t2830: f64, t10317: f64, t699: f64, t136: f64, t2826: f64, t41697: f64, t41701: f64, t41709: f64, t908: f64, t41640: f64, t41642: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41831 = t2403 * t2830;
    let t41833 = t699 * t10317;
    let t41836 = t136 * t2826 * t41697;
    let t41839 = t136 * t2826 * t41701;
    let t41842 = t136 * t908 * t41709;
    let t41845 = t136 * t908 * t41640;
    let t41855 = 0.11038e1_f64 * t41831 + 0.132456e1_f64 * t41833 - 0.99342e0_f64 * t41836 - 0.82785e-1_f64 * t41839 + 0.198684e1_f64 * t41842 + 0.49671e0_f64 * t41845 + 0.181155e1_f64 * t41642 - 0.80513333333333333336e0_f64 * t41656 - 0.53675555555555555556e0_f64 * t41658 + 0.44729629629629629629e0_f64 * t41660 + 0.40256666666666666668e0_f64 * t41662 - 0.89459259259259259259e0_f64 * t41669 - 0.301925e0_f64 * t41673 + 0.16102666666666666667e1_f64 * t41675;
    (t41831, t41833, t41836, t41839, t41842, t41845, t41855)
}
