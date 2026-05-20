//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3688/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3688<F: Float>(t1263: F, t21082: F, t3584: F, t5819: F, t1042: F, t1122: F, t1261: F, t12956: F, t17202: F, t17235: F, t17763: F, t20811: F, t20868: F, t20907: F, t20914: F, t3647: F, t3711: F, t5268: F, t5270: F, t5279: F, t5304: F, t57053: F, t60834: F, t60838: F, t65370: F) -> (F, F) {
    let t69742 = t1263 * t21082;
    let t69763 = t5819 * t3584;
    let t69770 = F::cast_from(0.28582678745379824648e-2_f64) * t3647 * t20868 + F::cast_from(0.95275595817932748826e-3_f64) * t17763 * t5304 - F::cast_from(0.63517063878621832552e-3_f64) * t1261 * t1042 * t17235 * t65370 - F::cast_from(0.11433071498151929859e-2_f64) * t17763 * t5270 + F::cast_from(0.28582678745379824648e-3_f64) * t12956 * t20811 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t69742 * t1122 - F::cast_from(0.57165357490759649296e-3_f64) * t3647 * t20907 - F::cast_from(0.57165357490759649296e-3_f64) * t1261 * t1042 * t5268 * t60838 - F::cast_from(0.28582678745379824648e-3_f64) * t1261 * t1042 * t5268 * t60834 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t17202 * t65370 + F::cast_from(0.57165357490759649296e-3_f64) * t12956 * t20914 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t5268 * t69763 + F::cast_from(0.57165357490759649296e-3_f64) * t57053 * t5279;
    (t69763, t69770)
}
