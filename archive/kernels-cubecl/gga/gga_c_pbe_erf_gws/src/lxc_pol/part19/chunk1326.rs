//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1326/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1326<F: Float>(t12080: F, t14101: F, t14058: F, t3871: F, t11995: F, t14567: F, t2080: F, t11807: F, t14063: F, t854: F, t3258: F, t3786: F, t850: F) -> (F, F, F, F, F) {
    let t57210 = t14101 * t12080;
    let t57213 = t14058 * t3871;
    let t57216 = t2080 * t11995 * t14567;
    let t57218 = t14063 * t11807;
    let t57219 = t854 * t57218;
    let t57222 = t850 * t3258 * t3786;
    (t57210, t57213, t57216, t57219, t57222)
}
