//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1603/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603<F: Float>(t3599: F, t44169: F, t11239: F, t1204: F, t13041: F, t10326: F, t1042: F, t1214: F, t1261: F, t1264: F, t12705: F, t12855: F, t12933: F, t12938: F, t12945: F, t12956: F, t13048: F, t17202: F, t17235: F, t17454: F, t2258: F, t247: F, t3363: F, t3584: F, t3606: F, t3617: F, t3647: F, t3711: F, t3720: F, t43180: F, t43194: F, t43875: F, t5268: F, t5296: F, t5302: F) -> (F, F) {
    let t44170 = t44169 * t3599;
    let t44173 = t1204 * t11239;
    let t44174 = t44173 * t13041;
    let t44185 = F::cast_from(0.95275595817932748828e-3_f64) * t1261 * t1042 * t5302 * t43194 + F::cast_from(0.28582678745379824648e-2_f64) * t3647 * t12945 + F::cast_from(0.57165357490759649296e-3_f64) * t3711 * t1042 * t5296 * t10326 * t1214 + F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t5296 * t2258 * t3584 - F::cast_from(0.28582678745379824648e-2_f64) * t12956 * t12938 - F::cast_from(0.14291339372689912324e-2_f64) * t3711 * t1042 * t3617 * t3584 * t3363 - F::cast_from(0.11433071498151929859e-2_f64) * t1261 * t1042 * t5268 * t43194 - F::cast_from(0.51448821741683684366e-2_f64) * t1261 * t1042 * t17202 * t43180 + F::cast_from(0.17149607247227894789e-2_f64) * t12956 * t12933 - F::cast_from(0.38110238327173099532e-2_f64) * t1261 * t1042 * t17235 * t43180 + F::cast_from(0.25724410870841842184e-2_f64) * t44170 * t3606 + F::cast_from(0.51448821741683684368e-2_f64) * t44174 * t13048 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t247 * t1264 * t43875 - F::cast_from(0.51448821741683684368e-2_f64) * t12855 * t3720 * t12705 * t17454;
    (t44173, t44185)
}
