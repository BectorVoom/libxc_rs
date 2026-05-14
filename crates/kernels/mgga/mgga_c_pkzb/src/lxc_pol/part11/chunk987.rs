//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 987/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk987<F: Float>(t17244: F, t5775: F, t659: F, t11817: F, t204: F, t208: F, t3981: F, t655: F) -> (F, F, F, F, F) {
    let t17245 = 1.0 / t17244;
    let t17329 = t659 * t5775;
    let t17348 = t204 * t11817 * t208;
    let t17349 = 0.96141975308641975307e-1 * t17348;
    let t17351 = t204 * t3981 * t655;
    (t17245, t17329, t17348, t17349, t17351)
}
