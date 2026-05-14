//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 663/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk663<F: Float>(t11437: F, t3194: F, t1909: F, t1820: F, t920: F, t1910: F, t3115: F, t8392: F, t1755: F, t1903: F, t1902: F, t1922: F, t452: F, t942: F, t3103: F, t499: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11438 = t3194 * t11437;
    let t11439 = t1909 * t11438;
    let t11442 = t920 * t1820;
    let t11443 = t1910 * t11442;
    let t11444 = t1909 * t11443;
    let t11448 = 2.0 / 27.0 * t8392 * t3115;
    let t11449 = t920 * t1755;
    let t11450 = t1903 * t11449;
    let t11451 = t1902 * t11450;
    let t11455 = t452 * t1922 * t942;
    let t11459 = t452 * t499 * t3103;
    (t11438, t11439, t11442, t11443, t11444, t11448, t11449, t11450, t11451, t11455, t11459)
}
