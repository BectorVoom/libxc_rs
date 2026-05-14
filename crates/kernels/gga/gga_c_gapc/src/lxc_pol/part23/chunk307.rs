//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 307/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk307<F: Float>(t1236: F, t374: F, t381: F, t372: F, t373: F, t1225: F, t54: F, t1179: F, t1184: F, t1191: F, t1206: F, t1214: F, t1218: F, t1222: F, t1229: F, t418: F, t76: F) -> (F, F, F, F, F, F, F, F) {
    let t1238 = t374 * t1236 * t381;
    let t1240 = 0.58482233974552040708e0 * t372 * t1238;
    let t1241 = t373 * t373;
    let t1242 = 1.0 / t1241;
    let t1243 = t1242 * t1225;
    let t1244 = t54 * t54;
    let t1245 = 1.0 / t1244;
    let t1246 = t1243 * t1245;
    let t1248 = 0.17315755899375863299e2 * t372 * t1246;
    let t1249 = -t1179 - t1184 - t1191 + t1206 + t1214 + t1218 + t1222 + t1229 - t1240 - t1248;
    let t1254 = t76 * t418;
    (t1238, t1240, t1242, t1245, t1246, t1248, t1249, t1254)
}
