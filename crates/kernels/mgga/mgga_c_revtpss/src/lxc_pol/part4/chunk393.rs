//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 393/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk393<F: Float>(t1221: F, t1222: F, t1227: F, t1231: F, t1235: F, t1238: F, t1247: F, t1252: F, t1258: F, t1261: F, t1266: F, t484: F) -> F {
    let t1269 = t1221 - t1222 * t1227 / F::new(288.0) + F::new(0.21437009059034868486e-3) * t1231 * t484 - F::new(0.21437009059034868486e-3) * t1235 * t1238 + F::new(0.21437009059034868486e-3) * t1247 * t1252 + t1258 - F::new(0.14291339372689912324e-3) * t1261 * t1266;
    t1269
}
