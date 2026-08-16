//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1576/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1576<F: Float>(t248: F, t3521: F, t5975: F, t1227: F, t1409: F, t15701: F, t15700: F, t3578: F, t1735: F, t4729: F, t18232: F, t4900: F) -> (F, F, F, F) {
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18395 = t15701 * t1409;
    let t18396 = t15700 * t18395;
    let t18397 = t3578 * t18396;
    let t18400 = t1735 * t4729;
    let t18401 = t3578 * t18400;
    let t18404 = t4900 * t18232;
    (t18393, t18397, t18401, t18404)
}
