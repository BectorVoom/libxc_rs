//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 596/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk596(t4443: f64, t4450: f64, t5668: f64, t5736: f64, t7738: f64, t7742: f64, t7746: f64, t7758: f64, t7765: f64, t7771: f64, t7773: f64, t7777: f64, t7780: f64, t7783: f64) -> f64 {
    let t8365 = -0.17648625e1_f64 * t7758 + 0.3529725e1_f64 * t7765 + t4443 + 0.34431666666666666666e0_f64 * t5668 - 0.34431666666666666667e0_f64 * t7738 + 0.103295e1_f64 * t7742 - 0.516475e0_f64 * t7746 + 0.31558125e0_f64 * t7771 + 0.6311625e0_f64 * t7773 + t4450 + 0.13892666666666666667e0_f64 * t5736 - 0.34731666666666666667e-1_f64 * t7777 + 0.20839e0_f64 * t7780 - 0.104195e0_f64 * t7783;
    t8365
}
