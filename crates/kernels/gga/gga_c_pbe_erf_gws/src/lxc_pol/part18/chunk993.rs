//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 993/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk993<F: Float>(t1192: F, t6781: F, t829: F, t830: F, t331: F, t816: F, t1195: F, t2242: F, t326: F, t837: F, t867: F) -> (F, F, F, F, F) {
    let t13937 = t6781 * t1192;
    let t13939 = t829 * t830 * t13937;
    let t13942 = t816 * t331;
    let t13948 = 35.0 / 432.0 * t2242 * t1195;
    let t13952 = t326 * t837;
    let t13953 = t13952 * t867;
    (t13939, t13942, t13948, t13952, t13953)
}
