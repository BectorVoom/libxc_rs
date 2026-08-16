//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3672/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672(t1179: f64, t20567: f64, t3520: f64, t6513: f64, t5142: f64, t3495: f64, t3476: f64, t6481: f64, t1161: f64, t1169: f64, t1189: f64, t17026: f64, t17089: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t3452: f64, t3472: f64, t3480: f64, t3498: f64, t3516: f64, t3524: f64, t45075: f64, t5143: f64, t5181: f64, t58234: f64, t58310: f64, t6506: f64, t68942: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t69028: f64, t69230: f64, t69246: f64, t69263: f64, t69279: f64, t69296: f64, t69312: f64, t69329: f64, t69345: f64) -> (f64, f64) {
    let t69354 = t20567 * t1179;
    let t69359 = t6513 * t3520;
    let t69367 = t5142 * t5142;
    let t69371 = t6513 * t3495;
    let t69376 = t6481 * t3476;
    let t69383 = 1.0_f64 * t1161 * (t69230 + t69246 + t69263 + t69279 + t69296 + t69312 + t69329 + t69345) * t1169 + 0.32163958997385070134e2_f64 * t45075 * t6506 + 0.11696447245269292414e1_f64 * t69354 * t1189 + 0.5848223622634646207e0_f64 * t20526 * t3516 + 0.17315859105681463759e2_f64 * t69359 * t3524 + 0.11696447245269292414e1_f64 * t58234 * t1757 + 0.23392894490538584828e1_f64 * t17089 * t5181 - 0.19751673498613801407e-1_f64 * t69028 - t68942 - 4.0_f64 * t3452 * t69367 * t1169 - t68946 - 0.11696447245269292414e1_f64 * t69371 * t3498 + 1.0_f64 * t20542 * t3472 + 0.32163958997385070134e2_f64 * t69376 * t3480 + 2.0_f64 * t58310 * t1745 + 4.0_f64 * t17026 * t5143 - t68949 - t68951 - t68954;
    (t69367, t69383)
}
