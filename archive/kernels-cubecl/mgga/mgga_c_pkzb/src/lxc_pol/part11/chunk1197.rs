//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1197/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1197<F: Float>(t10621: F, t164: F, t600: F, t10655: F, t5257: F, t10651: F, t16399: F, t10558: F, t1702: F, t10562: F, t16369: F, t1020: F, t2639: F) -> (F, F, F, F, F, F) {
    let t29248 = t10621 * t600 * t164;
    let t29252 = t5257 * t10655;
    let t29254 = t16399 * t10651;
    let t29262 = t1702 * t10558;
    let t29264 = t16369 * t10562;
    let t29279 = t1020 * t2639;
    (t29248, t29252, t29254, t29262, t29264, t29279)
}
