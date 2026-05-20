//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2590/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590<F: Float>(t45832: F, t460: F, t487: F, t5219: F, t5462: F, t1209: F, t21451: F, t17191: F, t3566: F, t3781: F, t5216: F, t45618: F) -> (F, F, F, F, F, F) {
    let t59737 = t460 * t45832 * t487;
    let t59749 = t5219 * t5462;
    let t59788 = t1209 * t21451;
    let t59817 = t3566 * t17191;
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    (t59737, t59749, t59788, t59817, t59854, t59864)
}
