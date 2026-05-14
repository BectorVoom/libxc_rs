//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1173/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1173<F: Float>(t2197: F, t2234: F, t3070: F, t2198: F, t6142: F, t8198: F, t1185: F, t6193: F, t18617: F, t6143: F, t8205: F, t22511: F, t22515: F, t22517: F, t22519: F, t22522: F, t22526: F, t22528: F, t22530: F, t22532: F, t22534: F, t22536: F, t22538: F, t22540: F, t22542: F, t22544: F, t22547: F) -> (F, F, F, F, F) {
    let t22550 = 6.0 * t2197 * t3070 * t2234;
    let t22553 = 0.28947563097646563121e3 * t6142 * t8198 * t2198;
    let t22556 = 2.0 * t2197 * t1185 * t6193;
    let t22559 = 0.62071215503128080361e4 * t18617 * t8205 * t6143;
    let t22560 = -t22511 + t22515 - t22517 - t22519 - t22522 - t22526 - t22528 - t22530 - t22532 - t22534 - t22536 + t22538 + t22540 + t22542 + t22544 - t22547 - t22550 - t22553 - t22556 - t22559;
    (t22550, t22553, t22556, t22559, t22560)
}
