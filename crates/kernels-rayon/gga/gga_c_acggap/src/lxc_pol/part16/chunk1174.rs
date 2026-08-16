//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1174/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1174(t6148: f64, t7822: f64, t30219: f64, t9720: f64, t4680: f64, t7575: f64, t9719: f64, t31603: f64, t31605: f64, t35814: f64, t35817: f64, t35828: f64, t37733: f64, t37736: f64, t40166: f64, t40168: f64, t40170: f64, t40172: f64, t40174: f64, t40179: f64, t40181: f64, t40183: f64) -> f64 {
    let t40185 = t7822 * t6148;
    let t40187 = t30219 * t9720;
    let t40190 = t7575 * t4680 * t9719;
    let t40192 = -0.17149607247227894789e-2_f64 * t40166 - 0.17149607247227894789e-2_f64 * t40168 - 0.40015750243531754508e-1_f64 * t40170 - 0.85748036236139473944e-3_f64 * t40172 - 0.85748036236139473944e-3_f64 * t40174 + 13.0_f64 / 288.0_f64 * t31603 + 0.19055119163586549765e-2_f64 * t31605 + 0.80031500487063509015e-2_f64 * t35814 + t35817 + t37733 + t35828 - t37736 - 0.85748036236139473944e-3_f64 * t40179 + 0.85748036236139473944e-3_f64 * t40181 - 0.85748036236139473944e-3_f64 * t40183 - 0.42874018118069736972e-3_f64 * t40185 + 0.31448092289604152068e-2_f64 * t40187 + 0.31448092289604152068e-2_f64 * t40190;
    t40192
}
