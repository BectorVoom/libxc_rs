//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1330/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1330<F: Float>(t22511: F, t22515: F, t22517: F, t22519: F, t22522: F, t22526: F, t22528: F, t22530: F, t22532: F, t22534: F, t22536: F, t2457: F, t955: F) -> (F, F) {
    let t23554 = -t22511 + t22515 - t22517 - t22519 - t22522 - t22526 - t22528 - t22530 - t22532 - t22534 - t22536;
    let t23555 = t2457 * t955;
    (t23554, t23555)
}
