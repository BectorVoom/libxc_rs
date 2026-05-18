//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 311/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk311<F: Float>(t54: F, t1243: F, t372: F, t1179: F, t1184: F, t1191: F, t1206: F, t1214: F, t1218: F, t1222: F, t1229: F, t1240: F) -> (F, F, F, F) {
    let t1244 = t54 * t54;
    let t1245 = F::new(1.0) / t1244;
    let t1246 = t1243 * t1245;
    let t1248 = F::new(0.17315755899375863299e2) * t372 * t1246;
    let t1249 = -t1179 - t1184 - t1191 + t1206 + t1214 + t1218 + t1222 + t1229 - t1240 - t1248;
    (t1245, t1246, t1248, t1249)
}
