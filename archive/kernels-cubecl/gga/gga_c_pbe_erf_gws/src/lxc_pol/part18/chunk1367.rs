//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1367/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1367<F: Float>(t11422: F, t13917: F, t53447: F, t15342: F, t53891: F, t11651: F, t13776: F, t52906: F, t14657: F, t54595: F, t14135: F, t3916: F) -> (F, F, F, F, F) {
    let t57495 = t13917 * t53447 * t11422;
    let t57497 = t53891 * t15342;
    let t57500 = t13776 * t52906 * t11651;
    let t57506 = t14657 * t54595;
    let t57508 = t3916 * t14135;
    (t57495, t57497, t57500, t57506, t57508)
}
