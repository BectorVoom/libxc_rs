//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2724/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724<F: Float>(t3666: F, t6594: F, t17283: F, t5362: F, t1222: F, t140: F, t21209: F, t21213: F, t3685: F, t12865: F, t5436: F, t3671: F, t371: F, t6609: F, t676: F) -> (F, F, F, F, F, F) {
    let t70469 = t3666 * t6594;
    let t70476 = t17283 * t5362;
    let t70491 = t1222 * t140 * t21209;
    let t70493 = t21213 * t3685;
    let t70496 = t5436 * t12865;
    let t70511 = t3671 * t371 * t676 * t6609;
    (t70469, t70476, t70491, t70493, t70496, t70511)
}
