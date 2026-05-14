//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 953/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk953<F: Float>(t19123: F, t5895: F, t13220: F, t459: F, t19109: F, t19119: F, t1390: F, t2191: F, t3278: F, t3544: F, t3283: F, t5943: F, t3532: F, t13153: F, t1354: F, t3564: F) -> (F, F, F, F, F, F, F) {
    let t19327 = t5895 * t19123;
    let t19330 = t13220 * t459;
    let t19331 = t19330 * t19109;
    let t19334 = t5895 * t19119;
    let t19338 = t2191 * t1390 * t3278;
    let t19339 = t3544 * t19338;
    let t19342 = t5943 * t3283;
    let t19343 = t3544 * t19342;
    let t19346 = t2191 * t3532;
    let t19347 = t19346 * t3278;
    let t19348 = t13153 * t19347;
    let t19351 = t3564 * t1354;
    (t19327, t19331, t19334, t19339, t19343, t19348, t19351)
}
