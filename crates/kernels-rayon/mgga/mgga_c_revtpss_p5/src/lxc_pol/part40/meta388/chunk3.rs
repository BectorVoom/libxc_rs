//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1403/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1403(t1188: f64, t17150: f64, t1749: f64, t3495: f64, t1161: f64, t1180: f64, t1189: f64, t12418: f64, t12476: f64, t17032: f64, t17086: f64, t17089: f64, t17094: f64, t17097: f64, t1745: f64, t1757: f64, t3447: f64, t3472: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3524: f64, t5120: f64, t5143: f64, t5158: f64, t5181: f64) -> f64 {
    let t17151 = t17150 * t1188;
    let t17154 = t1749 * t3495;
    let t17157 = 1.0_f64 * t5120 * t3472 + 0.32163958997385070134e2_f64 * t17032 * t3480 + 1.0_f64 * t12418 * t1745 + 2.0_f64 * t3447 * t5143 + 1.0_f64 * t1161 * t17086 + 0.11696447245269292414e1_f64 * t17089 * t1189 + t17094 + 0.5848223622634646207e0_f64 * t5158 * t3516 + 0.17315859105681463759e2_f64 * t17097 * t3524 + 0.5848223622634646207e0_f64 * t12476 * t1757 + 0.11696447245269292414e1_f64 * t3491 * t5181 + 0.5848223622634646207e0_f64 * t1180 * t17151 - 0.11696447245269292414e1_f64 * t17154 * t3498;
    t17157
}
