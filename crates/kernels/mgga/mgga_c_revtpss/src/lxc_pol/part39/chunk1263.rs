//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1263/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1263<F: Float>(t1188: F, t17150: F, t1749: F, t3495: F, t1161: F, t1180: F, t1189: F, t12418: F, t12476: F, t17032: F, t17086: F, t17089: F, t17094: F, t17097: F, t1745: F, t1757: F, t3447: F, t3472: F, t3480: F, t3491: F, t3498: F, t3516: F, t3524: F, t5120: F, t5143: F, t5158: F, t5181: F) -> (F,) {
    let t17151 = t17150 * t1188;
    let t17154 = t1749 * t3495;
    let t17157 = 1.0 * t5120 * t3472 + 0.32163958997385070134e2 * t17032 * t3480 + 1.0 * t12418 * t1745 + 2.0 * t3447 * t5143 + 1.0 * t1161 * t17086 + 0.11696447245269292414e1 * t17089 * t1189 + t17094 + 0.5848223622634646207e0 * t5158 * t3516 + 0.17315859105681463759e2 * t17097 * t3524 + 0.5848223622634646207e0 * t12476 * t1757 + 0.11696447245269292414e1 * t3491 * t5181 + 0.5848223622634646207e0 * t1180 * t17151 - 0.11696447245269292414e1 * t17154 * t3498;
    (t17157,)
}
