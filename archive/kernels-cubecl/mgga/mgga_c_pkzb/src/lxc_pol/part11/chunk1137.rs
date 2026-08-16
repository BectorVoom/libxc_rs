//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1137/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1137<F: Float>(t2096: F, t9616: F, t5713: F, t9605: F, t2038: F, t3656: F, t5939: F, t179: F, t299: F, t3515: F, t5672: F, t771: F, t9628: F) -> (F, F, F, F, F) {
    let t25530 = t2096 * t9616;
    let t25553 = t5713 * t9605;
    let t25556 = t2038 * t5939 * t3656;
    let t25572 = t299 * t179 * t5672 * t3515;
    let t25576 = t771 * t9628;
    (t25530, t25553, t25556, t25572, t25576)
}
