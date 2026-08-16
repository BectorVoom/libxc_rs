//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 912/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk912(t1188: f64, t5180: f64, t1756: f64, t3523: f64, t1187: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t1745: f64, t1757: f64, t3447: f64, t3452: f64, t3477: f64, t3491: f64, t3496: f64, t3521: f64, t435: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5117: f64, t5120: f64, t5125: f64, t5143: f64, t5147: f64, t5156: f64, t5158: f64, t5163: f64) -> (f64, f64, f64, f64) {
    let t5181 = t5180 * t1188;
    let t5184 = t1756 * t3523;
    let t5185 = t5184 * t1187;
    let t5188 = -0.310907e-1_f64 * t5117 * t435 + 1.0_f64 * t5120 * t1170 + 1.0_f64 * t3447 * t1745 - 2.0_f64 * t3452 * t5125 + 1.0_f64 * t1161 * t5143 + 0.32163958997385070134e2_f64 * t3477 * t5147 + t5062 - t5065 - t5067 + t5070 - t5107 - t5111 - 0.19751673498613801407e-1_f64 * t5156 + 0.5848223622634646207e0_f64 * t5158 * t1189 + 0.5848223622634646207e0_f64 * t3491 * t1757 - 0.11696447245269292414e1_f64 * t3496 * t5163 + 0.5848223622634646207e0_f64 * t1180 * t5181 + 0.17315859105681463759e2_f64 * t3521 * t5185;
    (t5181, t5184, t5185, t5188)
}
