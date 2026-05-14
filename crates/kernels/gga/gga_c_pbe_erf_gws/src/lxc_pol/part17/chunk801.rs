//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 801/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk801<F: Float>(t2749: F, t633: F, t5338: F, t5347: F, t1879: F, t2800: F, t219: F, t641: F, t1697: F, t422: F, t954: F, t617: F, t5211: F, t1639: F, t1642: F, t5219: F, t995: F) -> (F, F, F, F, F, F, F, F) {
    let t7478 = 8.0 / 45.0 * t633 * t2749;
    let t7479 = 16.0 / 45.0 * t5338;
    let t7480 = 8.0 / 45.0 * t5347;
    let t7482 = 8.0 / 15.0 * t1879 * t2800;
    let t7483 = t641 * t219;
    let t7484 = t7483 * t1697;
    let t7485 = t954 * t422;
    let t7486 = t7485 * t617;
    let t7487 = t7484 * t7486;
    let t7489 = 32.0 / 45.0 * t5211 * t7487;
    let t7490 = t1639 * t219;
    let t7491 = t7490 * t1642;
    let t7492 = t7491 * t7486;
    let t7494 = 16.0 / 27.0 * t5211 * t7492;
    let t7495 = t5219 * t995;
    (t7478, t7479, t7480, t7482, t7483, t7489, t7494, t7495)
}
