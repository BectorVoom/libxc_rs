//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 971/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk971<F: Float>(t6101: F, t7998: F, t45: F, t7970: F, t4083: F, t7959: F, t1253: F, t6078: F, t6082: F, t13682: F, t7927: F, t7928: F, t2129: F, t7963: F, t7960: F, t1275: F, t6119: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26336 = t7998 * t6101;
    let t26341 = t45 * t7970;
    let t26344 = t7959 * t4083;
    let t26345 = t26344 * t1253;
    let t26348 = t6082 * t6078;
    let t26351 = t7927 * t13682;
    let t26352 = t26351 * t1253;
    let t26359 = t7928 * t1253;
    let t26362 = t2129 * t6078;
    let t26365 = t7963 * t1253;
    let t26368 = t7960 * t1253;
    let t26373 = t1275 * t6119;
    (t26336, t26341, t26345, t26348, t26352, t26359, t26362, t26365, t26368, t26373)
}
