//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 236/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk236<F: Float>(t245: F, t410: F, t171: F, t977: F, t417: F, t978: F, t971: F, t1038: F, t1041: F, t1050: F, t1054: F, t1055: F, t1061: F, t1063: F, t1073: F, t1078: F, t1081: F, t1087: F, t1094: F, t1104: F, t1112: F, t167: F, t180: F, t396: F, t403: F, t411: F, t418: F, t5: F, t959: F) -> (F, F, F, F, F, F, F) {
    let t1116 = t245 * t410;
    let t1120 = t171 * t977;
    let t1121 = t978 * t417;
    let t1124 = t971 * t417;
    let t1127 = t171 * t1038;
    let t1128 = t978 * t1041;
    let t1131 = -F::cast_from(0.70983522622222222221e-3_f64) * t5 * t959 * t167 - F::cast_from(0.34246666666666666666e-1_f64) * t1054 * t1055 * t403 - F::new(2.0) * t1061 * t1063 + F::new(1.0) * t396 * t1073 + F::cast_from(0.32163958997385070134e2_f64) * t1078 * t1081 + t1050 + t1087 + t1094 - t1104 - t1112 - F::cast_from(0.24415263074675393405e-3_f64) * t5 * t959 * t180 - F::cast_from(0.10843581300301739842e-1_f64) * t1054 * t1116 * t418 - F::cast_from(0.11696447245269292414e1_f64) * t1120 * t1121 + F::cast_from(0.5848223622634646207e0_f64) * t411 * t1124 + F::cast_from(0.17315859105681463759e2_f64) * t1127 * t1128;
    (t1116, t1120, t1121, t1124, t1127, t1128, t1131)
}
