//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1258/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1258(t11228: f64, t25756: f64, t35506: f64, t35510: f64, t35512: f64, t35515: f64, t35519: f64, t35521: f64, t35524: f64, t35527: f64, t35531: f64, t35533: f64, t35536: f64, t35539: f64, t35543: f64) -> f64 {
    let t35545 = t11228 * t25756;
    let t35547 = 0.12653481940368541265e-5_f64 * t35506 + 0.7381197798548315738e-6_f64 * t35510 - 0.86898242813537603824e-4_f64 * t35512 + 0.5431140175846100239e-5_f64 * t35515 - 0.5431140175846100239e-5_f64 * t35519 + 0.59742541934307102628e-4_f64 * t35521 - 0.5431140175846100239e-5_f64 * t35524 - 0.27155700879230501195e-5_f64 * t35527 - 0.3218855744218122075e-6_f64 * t35531 - 0.10010310157269334868e-3_f64 * t35533 + 0.27155700879230501195e-5_f64 * t35536 + 0.3218855744218122075e-6_f64 * t35539 + 0.70412469404771420391e-7_f64 * t35543 - 0.2530696388073708253e-5_f64 * t35545;
    t35547
}
