//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 904/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk904<F: Float>(t1217: F, t806: F, t1218: F, t2358: F, t1216: F, t298: F, t2362: F, t40: F, t1000: F, t6635: F, t1257: F, t1256: F, t305: F) -> (F, F, F, F, F, F) {
    let t8320 = t1217 * t806;
    let t8323 = t2358 * t1218;
    let t8326 = t298 * t1216;
    let t8329 = t2362 * t40;
    let t8336 = t6635 * t1000;
    let t8337 = t8336 * t1257;
    let t8340 = t305 * t1256;
    (t8320, t8323, t8326, t8329, t8337, t8340)
}
