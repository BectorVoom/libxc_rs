//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3688/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3688(t1263: f64, t21082: f64, t3584: f64, t5819: f64, t1042: f64, t1122: f64, t1261: f64, t12956: f64, t17202: f64, t17235: f64, t17763: f64, t20811: f64, t20868: f64, t20907: f64, t20914: f64, t3647: f64, t3711: f64, t5268: f64, t5270: f64, t5279: f64, t5304: f64, t57053: f64, t60834: f64, t60838: f64, t65370: f64) -> (f64, f64) {
    let t69742 = t1263 * t21082;
    let t69763 = t5819 * t3584;
    let t69770 = 0.28582678745379824648e-2_f64 * t3647 * t20868 + 0.95275595817932748826e-3_f64 * t17763 * t5304 - 0.63517063878621832552e-3_f64 * t1261 * t1042 * t17235 * t65370 - 0.11433071498151929859e-2_f64 * t17763 * t5270 + 0.28582678745379824648e-3_f64 * t12956 * t20811 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t69742 * t1122 - 0.57165357490759649296e-3_f64 * t3647 * t20907 - 0.57165357490759649296e-3_f64 * t1261 * t1042 * t5268 * t60838 - 0.28582678745379824648e-3_f64 * t1261 * t1042 * t5268 * t60834 - 0.85748036236139473944e-3_f64 * t1261 * t1042 * t17202 * t65370 + 0.57165357490759649296e-3_f64 * t12956 * t20914 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t5268 * t69763 + 0.57165357490759649296e-3_f64 * t57053 * t5279;
    (t69763, t69770)
}
