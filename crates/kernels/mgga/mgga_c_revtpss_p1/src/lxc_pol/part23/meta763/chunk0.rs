//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2559/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2559<F: Float>(t16543: F, t3046: F, t4746: F, t4995: F, t15669: F, t3286: F, t1651: F, t378: F, t342: F, t43400: F, t3057: F, t12077: F, t1647: F) -> (F, F, F, F, F, F, F) {
    let t55701 = t3046 * t16543;
    let t55732 = t4746 * t4995;
    let t55747 = t15669 * t3286;
    let t55764 = t378 * t1651;
    let t55805 = t342 * t43400 * t378;
    let t55887 = t3057 * t16543;
    let t55899 = t1647 * t12077;
    (t55701, t55732, t55747, t55764, t55805, t55887, t55899)
}
