//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 410/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk410(t1236: f64, t381: f64, t1242: f64, t46: f64, t1225: f64, t1245: f64, t1179: f64, t1180: f64, t1184: f64, t1191: f64, t1206: f64, t1214: f64, t2084: f64, t2090: f64, t2092: f64, t2102: f64, t2107: f64, t2110: f64, t2116: f64, t2120: f64, t2121: f64, t242: f64, t4: f64, t55: f64, t706: f64, t713: f64, t719: f64, t720: f64, t96: f64) -> f64 {
    let t2124 = t1236 * t381;
    let t2127 = t46 * t1242;
    let t2128 = t1225 * t1245;
    let t2131 = -0.70981924444444444442e-3_f64 * t4 * t96 * t242 - 0.34246666666666666666e-1_f64 * t1180 * t2084 * t713 - 2.0_f64 * t2090 * t2092 + 1.0_f64 * t706 * t2102 + 0.32164683177870697974e2_f64 * t2107 * t2110 + t1179 + t1184 + t1191 - t1206 - t1214 - 0.24415406715670879921e-3_f64 * t4 * t96 * t55 - 0.10843580882781524214e-1_f64 * t1180 * t2116 * t720 - 0.11696446794910408142e1_f64 * t2120 * t2121 + 0.58482233974552040708e0_f64 * t719 * t2124 + 0.17315755899375863299e2_f64 * t2127 * t2128;
    t2131
}
