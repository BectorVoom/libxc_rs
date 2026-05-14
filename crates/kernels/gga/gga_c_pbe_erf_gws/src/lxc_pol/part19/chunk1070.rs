//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1070/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1070<F: Float>(t3306: F, t8589: F, t2395: F, t3703: F, t19894: F, t3912: F, t8546: F, t944: F, t2416: F, t3906: F, t13252: F, t36888: F, t274: F, t3111: F, t9607: F, t1123: F, t745: F) -> (F, F, F, F, F, F, F, F) {
    let t39460 = t8589 * t3306;
    let t39579 = t2395 * t3703;
    let t39689 = t3912 * t19894;
    let t43260 = t8546 * t944;
    let t43526 = t3906 * t2416;
    let t44196 = t36888 * t13252;
    let t44200 = t3111 * t274;
    let t44201 = t9607 * t44200;
    let t44205 = t1123 * t745;
    (t39460, t39579, t39689, t43260, t43526, t44196, t44201, t44205)
}
