//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1279/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1279<F: Float>(t15362: F, t9270: F, t53841: F, t53923: F, t9942: F, t11354: F, t14797: F, t3989: F, t3990: F, t15134: F, t51563: F, t1161: F, t274: F) -> (F, F, F, F, F) {
    let t56228 = t9270 * t15362;
    let t56236 = t53923 * t53841 * t9942;
    let t56240 = t3989 * t3990 * t14797 * t11354;
    let t56242 = t51563 * t15134;
    let t56246 = t274 * t1161;
    (t56228, t56236, t56240, t56242, t56246)
}
