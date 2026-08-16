//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1603/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603(t3599: f64, t44169: f64, t11239: f64, t1204: f64, t13041: f64, t10326: f64, t1042: f64, t1214: f64, t1261: f64, t1264: f64, t12705: f64, t12855: f64, t12933: f64, t12938: f64, t12945: f64, t12956: f64, t13048: f64, t17202: f64, t17235: f64, t17454: f64, t2258: f64, t247: f64, t3363: f64, t3584: f64, t3606: f64, t3617: f64, t3647: f64, t3711: f64, t3720: f64, t43180: f64, t43194: f64, t43875: f64, t5268: f64, t5296: f64, t5302: f64) -> (f64, f64) {
    let t44170 = t44169 * t3599;
    let t44173 = t1204 * t11239;
    let t44174 = t44173 * t13041;
    let t44185 = 0.95275595817932748828e-3_f64 * t1261 * t1042 * t5302 * t43194 + 0.28582678745379824648e-2_f64 * t3647 * t12945 + 0.57165357490759649296e-3_f64 * t3711 * t1042 * t5296 * t10326 * t1214 + 0.85748036236139473944e-3_f64 * t3711 * t1042 * t5296 * t2258 * t3584 - 0.28582678745379824648e-2_f64 * t12956 * t12938 - 0.14291339372689912324e-2_f64 * t3711 * t1042 * t3617 * t3584 * t3363 - 0.11433071498151929859e-2_f64 * t1261 * t1042 * t5268 * t43194 - 0.51448821741683684366e-2_f64 * t1261 * t1042 * t17202 * t43180 + 0.17149607247227894789e-2_f64 * t12956 * t12933 - 0.38110238327173099532e-2_f64 * t1261 * t1042 * t17235 * t43180 + 0.25724410870841842184e-2_f64 * t44170 * t3606 + 0.51448821741683684368e-2_f64 * t44174 * t13048 - 0.85748036236139473944e-3_f64 * t1261 * t247 * t1264 * t43875 - 0.51448821741683684368e-2_f64 * t12855 * t3720 * t12705 * t17454;
    (t44173, t44185)
}
