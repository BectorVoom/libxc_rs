//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 832/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk832<F: Float>(t1188: F, t5180: F, t1756: F, t3523: F, t1187: F, t1161: F, t1170: F, t1180: F, t1189: F, t1745: F, t1757: F, t3447: F, t3452: F, t3477: F, t3491: F, t3496: F, t3521: F, t435: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5117: F, t5120: F, t5125: F, t5143: F, t5147: F, t5156: F, t5158: F, t5163: F) -> (F, F, F, F) {
    let t5181 = t5180 * t1188;
    let t5184 = t1756 * t3523;
    let t5185 = t5184 * t1187;
    let t5188 = -0.310907e-1 * t5117 * t435 + 1.0 * t5120 * t1170 + 1.0 * t3447 * t1745 - 2.0 * t3452 * t5125 + 1.0 * t1161 * t5143 + 0.32163958997385070134e2 * t3477 * t5147 + t5062 - t5065 - t5067 + t5070 - t5107 - t5111 - 0.19751673498613801407e-1 * t5156 + 0.5848223622634646207e0 * t5158 * t1189 + 0.5848223622634646207e0 * t3491 * t1757 - 0.11696447245269292414e1 * t3496 * t5163 + 0.5848223622634646207e0 * t1180 * t5181 + 0.17315859105681463759e2 * t3521 * t5185;
    (t5181, t5184, t5185, t5188)
}
