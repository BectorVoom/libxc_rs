//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1316/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1316<F: Float>(t3214: F, t6531: F, t6488: F, t8368: F, t2380: F, t2383: F, t3236: F, t54: F, t6491: F, t8360: F, t1238: F, t6400: F) -> (F, F, F, F, F) {
    let t23248 = t3214 * t6531;
    let t23250 = t8368 * t6488;
    let t23254 = t2380 * t54 * t3236 * t2383;
    let t23264 = t8360 * t6491;
    let t23266 = t1238 * t6400;
    (t23248, t23250, t23254, t23264, t23266)
}
