//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1114/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1114<F: Float>(t20154: F, t3067: F, t4207: F, t938: F, t53970: F, t53975: F, t53985: F, t54429: F, t4227: F, t6781: F, t829: F, t830: F, t14886: F, t4386: F, t892: F, t15036: F, t19906: F) -> (F, F, F, F, F, F, F, F) {
    let t55734 = t20154 * t3067 * t4207 * t938;
    let t55739 = 7.0 / 72.0 * t53970;
    let t55741 = 7.0 / 288.0 * t53975;
    let t55745 = 7.0 / 36.0 * t53985;
    let t55752 = 7.0 / 72.0 * t54429;
    let t55762 = t6781 * t4227;
    let t55764 = t829 * t830 * t55762;
    let t55769 = t4386 * t892 * t14886;
    let t55773 = 7.0 / 72.0 * t19906 * t15036;
    (t55734, t55739, t55741, t55745, t55752, t55764, t55769, t55773)
}
