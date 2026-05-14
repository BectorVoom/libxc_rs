//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 200/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk200<F: Float>(t568: F, t616: F, t615: F, t578: F, t580: F, t583: F, t590: F, t603: F, t611: F, t612: F) -> (F, F) {
    let t617 = t616 * t568;
    let t618 = t615 * t617;
    let t621 = -t578 - t580 * t583 / 48.0 - 0.21437009059034868486e-3 * t590 * t603 - t611 - 0.85748036236139473944e-3 * t612 * t618;
    (t618, t621)
}
