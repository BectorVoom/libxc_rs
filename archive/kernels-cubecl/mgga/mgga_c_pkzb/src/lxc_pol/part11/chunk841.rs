//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 841/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk841<F: Float>(t179: F, t2593: F, t2653: F, t2600: F, t6758: F, t1020: F, t164: F, t2646: F, t1769: F, t3457: F, t50: F, t8817: F) -> (F, F, F, F, F, F) {
    let t8996 = t179 * t2593 * t2653;
    let t9000 = t179 * t2600 * t6758;
    let t9003 = t164 * t1020;
    let t9005 = t179 * t2646 * t9003;
    let t9008 = t1769 * t3457;
    let t9011 = t50 * t8817;
    (t8996, t9000, t9003, t9005, t9008, t9011)
}
