//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3128/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3128<F: Float>(t4746: F, t4995: F, t15669: F, t3286: F, t1651: F, t378: F, t342: F, t43400: F, t1086: F, t15886: F, t16543: F, t3057: F) -> (F, F, F, F, F, F) {
    let t55732 = t4746 * t4995;
    let t55747 = t15669 * t3286;
    let t55764 = t378 * t1651;
    let t55805 = t342 * t43400 * t378;
    let t55868 = t15886 * t1086;
    let t55887 = t3057 * t16543;
    (t55732, t55747, t55764, t55805, t55868, t55887)
}
