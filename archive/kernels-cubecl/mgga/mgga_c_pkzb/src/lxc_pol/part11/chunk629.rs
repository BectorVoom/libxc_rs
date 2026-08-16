//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 629/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk629<F: Float>(t3532: F, t665: F, t1873: F, t3528: F, t672: F, t1066: F) -> (F, F, F, F) {
    let t3533 = t665 * t3532;
    let t3537 = t1873 * t3528;
    let t3539 = t672 * t3532;
    let t3542 = t1066 * t1066;
    (t3533, t3537, t3539, t3542)
}
