//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1105/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1105<F: Float>(t14193: F, t22493: F, t53060: F, t14185: F, t3306: F, t353: F, t859: F, t1105: F, t4111: F, t4386: F, t1206: F, t2494: F, t1144: F, t14191: F, t14180: F, t14949: F, t9270: F) -> (F, F, F, F, F, F, F, F) {
    let t54942 = 7.0 / 144.0 * t22493 * t14193;
    let t54946 = 7.0 / 288.0 * t53060;
    let t54952 = t859 * t353 * t14185 * t3306;
    let t54957 = t4386 * t353 * t4111 * t1105;
    let t54962 = t4386 * t353 * t1206 * t2494;
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = 7.0 / 72.0 * t9270 * t14949;
    (t54942, t54946, t54952, t54957, t54962, t54978, t54984, t54998)
}
