//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 264/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk264<F: Float>(t1253: F, t1254: F, t1222: F, t1227: F, t365: F, t45: F) -> (F, F, F, F) {
    let t1255 = t1253 * t1254;
    let t1258 = 0.92708333333333333333e-2 * t1222;
    let t1260 = -t1258 - 0.92708333333333333333e-2 * t1227;
    let t1264 = t45 * t365;
    (t1255, t1258, t1260, t1264)
}
