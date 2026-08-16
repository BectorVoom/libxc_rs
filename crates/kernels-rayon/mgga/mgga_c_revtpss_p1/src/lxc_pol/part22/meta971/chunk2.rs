//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3246/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246(t6016: f64, t853: f64, t2661: f64, t2662: f64, t2749: f64, t18392: f64, t2477: f64, t40374: f64, t40393: f64, t40395: f64, t40399: f64, t40409: f64, t40411: f64, t50353: f64, t50370: f64, t50372: f64, t50374: f64, t775: f64, t828: f64, t851: f64) -> f64 {
    let t61579 = t853 * t6016;
    let t61582 = t2661 * t2662 * t61579 * t2749;
    let t61599 = -0.57165357490759649296e-4_f64 * t61582 + 0.85748036236139473944e-2_f64 * t851 * t2477 * t828 * t18392 * t775 + 0.80031500487063509015e-2_f64 * t50353 + 0.13552000749142754193e-3_f64 * t40374 - 0.56688979511669985553e-2_f64 * t40393 - 0.56688979511669985553e-2_f64 * t40395 + 0.11337795902333997111e-1_f64 * t40399 - 0.40164115440237189888e-6_f64 * t40409 + 0.60976381323476959248e-3_f64 * t40411 + 0.30234122406223992295e0_f64 * t50370 + 0.14450132032386466905e-2_f64 * t50372 - 0.30488190661738479624e-3_f64 * t50374;
    t61599
}
