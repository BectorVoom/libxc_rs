//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1176/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1176<F: Float>(t2389: F, t5939: F, t918: F, t17928: F, t2362: F, t326: F, t17932: F, t401: F, t913: F, t2367: F, t2372: F, t2395: F, t2397: F, t395: F, t6512: F, t2370: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19067 = t918 * t5939 * t2389;
    let t19078 = t17928 * t2362;
    let t19079 = t19078 * t326;
    let t19080 = t401 * t17932;
    let t19090 = t17928 * t913;
    let t19091 = t19090 * t326;
    let t19099 = t2367 * t5939 * t2372;
    let t19102 = t2395 * t5939 * t2397;
    let t19106 = t17928 / t6512 / t395;
    let t19107 = t19106 * t326;
    let t19109 = t2370 * t2370;
    (t19067, t19078, t19079, t19080, t19090, t19091, t19099, t19102, t19106, t19107, t19109)
}
