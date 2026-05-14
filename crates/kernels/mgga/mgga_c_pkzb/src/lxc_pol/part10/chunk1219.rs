//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1219/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1219<F: Float>(t3185: F, t8381: F, t926: F, t8423: F, t8428: F, t8431: F, t54: F, t8253: F, t8260: F, t3214: F, t6531: F, t6488: F, t8368: F, t2380: F, t2383: F, t3236: F) -> (F, F, F, F, F, F, F, F) {
    let t23201 = t3185 * t926 * t8381;
    let t23204 = t3185 * t926 * t8423;
    let t23207 = t8428 * t926 * t8431;
    let t23213 = t54 * t8253;
    let t23215 = t3185 * t23213 * t8260;
    let t23248 = t3214 * t6531;
    let t23250 = t8368 * t6488;
    let t23254 = t2380 * t54 * t3236 * t2383;
    (t23201, t23204, t23207, t23213, t23215, t23248, t23250, t23254)
}
