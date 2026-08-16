//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 410/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk410<F: Float>(t1236: F, t381: F, t1242: F, t46: F, t1225: F, t1245: F, t1179: F, t1180: F, t1184: F, t1191: F, t1206: F, t1214: F, t2084: F, t2090: F, t2092: F, t2102: F, t2107: F, t2110: F, t2116: F, t2120: F, t2121: F, t242: F, t4: F, t55: F, t706: F, t713: F, t719: F, t720: F, t96: F) -> F {
    let t2124 = t1236 * t381;
    let t2127 = t46 * t1242;
    let t2128 = t1225 * t1245;
    let t2131 = -F::cast_from(0.70981924444444444442e-3_f64) * t4 * t96 * t242 - F::cast_from(0.34246666666666666666e-1_f64) * t1180 * t2084 * t713 - F::cast_from(2.0_f64) * t2090 * t2092 + F::cast_from(1.0_f64) * t706 * t2102 + F::cast_from(0.32164683177870697974e2_f64) * t2107 * t2110 + t1179 + t1184 + t1191 - t1206 - t1214 - F::cast_from(0.24415406715670879921e-3_f64) * t4 * t96 * t55 - F::cast_from(0.10843580882781524214e-1_f64) * t1180 * t2116 * t720 - F::cast_from(0.11696446794910408142e1_f64) * t2120 * t2121 + F::cast_from(0.58482233974552040708e0_f64) * t719 * t2124 + F::cast_from(0.17315755899375863299e2_f64) * t2127 * t2128;
    t2131
}
