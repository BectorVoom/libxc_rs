//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3108/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3108<F: Float>(t11875: F, t11922: F, t15605: F, t11852: F, t41270: F, t15905: F, t43384: F, t15595: F, t3091: F, t43131: F, t11675: F, t15984: F) -> (F, F, F, F, F) {
    let t54533 = t11875 * t11922 * t15605;
    let t54537 = t11852 * t41270;
    let t54542 = t43384 * t15905;
    let t54546 = t3091 * t43131 * t15595;
    let t54550 = t11675 * t15984;
    (t54533, t54537, t54542, t54546, t54550)
}
