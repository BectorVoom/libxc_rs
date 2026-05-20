//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3672/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3672<F: Float>(t1179: F, t20567: F, t3520: F, t6513: F, t5142: F, t3495: F, t3476: F, t6481: F, t1161: F, t1169: F, t1189: F, t17026: F, t17089: F, t1745: F, t1757: F, t20526: F, t20542: F, t3452: F, t3472: F, t3480: F, t3498: F, t3516: F, t3524: F, t45075: F, t5143: F, t5181: F, t58234: F, t58310: F, t6506: F, t68942: F, t68946: F, t68949: F, t68951: F, t68954: F, t69028: F, t69230: F, t69246: F, t69263: F, t69279: F, t69296: F, t69312: F, t69329: F, t69345: F) -> (F, F) {
    let t69354 = t20567 * t1179;
    let t69359 = t6513 * t3520;
    let t69367 = t5142 * t5142;
    let t69371 = t6513 * t3495;
    let t69376 = t6481 * t3476;
    let t69383 = F::new(1.0) * t1161 * (t69230 + t69246 + t69263 + t69279 + t69296 + t69312 + t69329 + t69345) * t1169 + F::cast_from(0.32163958997385070134e2_f64) * t45075 * t6506 + F::cast_from(0.11696447245269292414e1_f64) * t69354 * t1189 + F::cast_from(0.5848223622634646207e0_f64) * t20526 * t3516 + F::cast_from(0.17315859105681463759e2_f64) * t69359 * t3524 + F::cast_from(0.11696447245269292414e1_f64) * t58234 * t1757 + F::cast_from(0.23392894490538584828e1_f64) * t17089 * t5181 - F::cast_from(0.19751673498613801407e-1_f64) * t69028 - t68942 - F::new(4.0) * t3452 * t69367 * t1169 - t68946 - F::cast_from(0.11696447245269292414e1_f64) * t69371 * t3498 + F::new(1.0) * t20542 * t3472 + F::cast_from(0.32163958997385070134e2_f64) * t69376 * t3480 + F::new(2.0) * t58310 * t1745 + F::new(4.0) * t17026 * t5143 - t68949 - t68951 - t68954;
    (t69367, t69383)
}
