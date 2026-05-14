//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1367/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1367<F: Float>(t1292: F, t1308: F, t20886: F, t32019: F, t33593: F, t1411: F, t32203: F, t33600: F, t2153: F, t3502: F, t9461: F, t33376: F, t3969: F, t32176: F, t33377: F, t19040: F, t3759: F) -> (F, F, F, F, F, F, F) {
    let t114107 = t20886 * t1292 * t1308;
    let t114111 = 0.69444444444444444446e-2 * t32019 * t33593;
    let t114113 = t1411 * t32203 * t33600;
    let t114117 = t1411 * t9461 * t2153 * t3502;
    let t114121 = t33376 * t3969;
    let t114125 = 0.26805555555555555556e-2 * t33377 * t32176;
    let t114131 = t3759 * t9461 * t19040;
    (t114107, t114111, t114113, t114117, t114121, t114125, t114131)
}
