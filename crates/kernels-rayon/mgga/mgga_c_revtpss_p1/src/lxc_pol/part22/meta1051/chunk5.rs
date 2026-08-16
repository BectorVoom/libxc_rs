//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3710/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710(t13058: f64, t20786: f64, t11262: f64, t3711: f64, t6618: f64, t1261: f64, t21110: f64, t3172: f64, t1042: f64, t12784: f64, t17232: f64, t20792: f64, t21219: f64, t3647: f64, t3674: f64, t5268: f64, t5391: f64, t57063: f64, t57070: f64, t65433: f64, t70263: f64, t70265: f64, t70267: f64, t70270: f64, t70273: f64) -> f64 {
    let t70275 = t13058 * t20786;
    let t70278 = t3711 * t11262 * t6618;
    let t70281 = t1261 * t3172 * t21110;
    let t70289 = 0.47637797908966374414e-3_f64 * t3647 * t20792 - 0.57165357490759649296e-3_f64 * t1261 * t1042 * t5268 * t65433 + 0.47637797908966374413e-4_f64 * t70263 + 0.30488190661738479624e-2_f64 * t70265 - 0.45732285992607719436e-2_f64 * t70267 * t3674 - 0.76220476654346199061e-3_f64 * t70270 + 0.31758531939310916276e-3_f64 * t70273 - 0.28582678745379824648e-3_f64 * t70275 - 0.6351706387862183255e-4_f64 * t70278 - 0.8468941850482911007e-3_f64 * t70281 + 0.60976381323476959248e-2_f64 * t5391 * t17232 - 0.28582678745379824648e-3_f64 * t12784 * t21219 + 0.57165357490759649296e-3_f64 * t57063 + 0.11433071498151929859e-2_f64 * t57070;
    t70289
}
