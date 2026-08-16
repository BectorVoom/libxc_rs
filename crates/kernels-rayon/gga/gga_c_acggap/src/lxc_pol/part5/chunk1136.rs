//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1136/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1136(t1163: f64, t1165: f64, t1552: f64, t322: f64, t6151: f64, t1106: f64, t1181: f64, t1879: f64, t3391: f64, t3382: f64, t5981: f64, t1131: f64, t1180: f64, t12478: f64, t13268: f64, t13364: f64, t1531: f64, t1532: f64, t15410: f64, t15429: f64, t15431: f64, t17185: f64, t1753: f64, t19834: f64, t301: f64, t3462: f64, t3539: f64, t372: f64, t5615: f64, t5852: f64, t5922: f64, t8790: f64) -> f64 {
    let t20365 = t1163 * t1165 * t1552 * t6151 * t322;
    let t20379 = t3391 * t1181 * t1879 * t1106;
    let t20383 = t3382 * t5981;
    let t20385 = -0.13719685797782315831e-1_f64 * t17185 * t13364 * t8790 * t5615 * t301 - 0.85748036236139473944e-3_f64 * t1180 * t1165 * t1532 * t19834 - 0.17149607247227894789e-2_f64 * t15410 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t1552 * t1753 * t1131 - 0.17149607247227894789e-2_f64 * t20365 - 0.34299214494455789578e-2_f64 * t1531 * t1181 * t5922 * t3539 * t372 - 0.60023625365297631762e-2_f64 * t12478 - 0.17149607247227894789e-2_f64 * t3462 * t1165 * t5852 * t13268 - 0.51448821741683684368e-2_f64 * t20379 - 0.34299214494455789578e-2_f64 * t15429 - 0.17149607247227894789e-1_f64 * t15431 + 0.17149607247227894789e-2_f64 * t20383;
    t20385
}
