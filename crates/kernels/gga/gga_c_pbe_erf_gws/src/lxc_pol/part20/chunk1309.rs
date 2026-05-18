//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1309/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1309<F: Float>(t15342: F, t53774: F, t3888: F, t859: F, t13792: F, t52990: F, t13808: F, t15186: F, t13917: F, t343: F, t53799: F, t54590: F, t824: F) -> (F, F, F, F, F) {
    let t56740 = t53774 * t15342;
    let t56742 = t859 * t3888;
    let t56743 = t13792 * t56742;
    let t56745 = t52990 * t15342;
    let t56747 = t13808 * t15186;
    let t56753 = t13917 * t53799 * t824 * t54590 * t343;
    (t56740, t56743, t56745, t56747, t56753)
}
