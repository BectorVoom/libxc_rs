//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1228/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1228<F: Float>(t14617: F, t50943: F, t345: F, t6126: F, t4146: F, t51818: F, t14592: F, t50994: F, t1176: F, t14639: F, t6365: F, t923: F) -> (F, F, F, F, F) {
    let t53272 = t50943 * t14617;
    let t53283 = t345 * t6126;
    let t53334 = t51818 * t4146;
    let t53353 = t50994 * t14592;
    let t53424 = t1176 * t923 * t6365 * t14639;
    (t53272, t53283, t53334, t53353, t53424)
}
