//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 236/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk236(t245: f64, t410: f64, t171: f64, t977: f64, t417: f64, t978: f64, t971: f64, t1038: f64, t1041: f64, t1050: f64, t1054: f64, t1055: f64, t1061: f64, t1063: f64, t1073: f64, t1078: f64, t1081: f64, t1087: f64, t1094: f64, t1104: f64, t1112: f64, t167: f64, t180: f64, t396: f64, t403: f64, t411: f64, t418: f64, t5: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1116 = t245 * t410;
    let t1120 = t171 * t977;
    let t1121 = t978 * t417;
    let t1124 = t971 * t417;
    let t1127 = t171 * t1038;
    let t1128 = t978 * t1041;
    let t1131 = -0.70983522622222222221e-3_f64 * t5 * t959 * t167 - 0.34246666666666666666e-1_f64 * t1054 * t1055 * t403 - 2.0_f64 * t1061 * t1063 + 1.0_f64 * t396 * t1073 + 0.32163958997385070134e2_f64 * t1078 * t1081 + t1050 + t1087 + t1094 - t1104 - t1112 - 0.24415263074675393405e-3_f64 * t5 * t959 * t180 - 0.10843581300301739842e-1_f64 * t1054 * t1116 * t418 - 0.11696447245269292414e1_f64 * t1120 * t1121 + 0.5848223622634646207e0_f64 * t411 * t1124 + 0.17315859105681463759e2_f64 * t1127 * t1128;
    (t1116, t1120, t1121, t1124, t1127, t1128, t1131)
}
