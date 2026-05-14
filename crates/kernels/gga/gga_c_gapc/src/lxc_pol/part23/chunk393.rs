//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 393/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk393<F: Float>(t2101: F, t712: F, t704: F, t233: F, t241: F, t2091: F, t374: F, t78: F, t1224: F, t46: F, t1225: F, t381: F, t1236: F, t1242: F, t1245: F, t1179: F, t1180: F, t1184: F, t1191: F, t1206: F, t1214: F, t2084: F, t2090: F, t2092: F, t242: F, t4: F, t55: F, t706: F, t713: F, t719: F, t720: F, t96: F) -> (F,) {
    let t2102 = t2101 * t712;
    let t2105 = t704 * t704;
    let t2106 = 1.0 / t2105;
    let t2107 = t233 * t2106;
    let t2108 = t241 * t241;
    let t2109 = 1.0 / t2108;
    let t2110 = t2091 * t2109;
    let t2116 = t78 * t374;
    let t2120 = t46 * t1224;
    let t2121 = t1225 * t381;
    let t2124 = t1236 * t381;
    let t2127 = t46 * t1242;
    let t2128 = t1225 * t1245;
    let t2131 = -0.70981924444444444442e-3 * t4 * t96 * t242 - 0.34246666666666666666e-1 * t1180 * t2084 * t713 - 2.0 * t2090 * t2092 + 1.0 * t706 * t2102 + 0.32164683177870697974e2 * t2107 * t2110 + t1179 + t1184 + t1191 - t1206 - t1214 - 0.24415406715670879921e-3 * t4 * t96 * t55 - 0.10843580882781524214e-1 * t1180 * t2116 * t720 - 0.11696446794910408142e1 * t2120 * t2121 + 0.58482233974552040708e0 * t719 * t2124 + 0.17315755899375863299e2 * t2127 * t2128;
    (t2131,)
}
