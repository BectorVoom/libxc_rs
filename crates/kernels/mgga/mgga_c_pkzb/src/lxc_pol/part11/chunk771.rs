//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 771/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk771<F: Float>(t1220: F, t2349: F, t154: F, t2347: F, t3026: F, t385: F, t1167: F, t6446: F, t2344: F, t1235: F, t2023: F, t46: F, t2394: F, t1229: F, t5939: F, t918: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8325 = t1220 * t2349 / 54.0;
    let t8329 = t154 * t2347 * t3026;
    let t8331 = t385 * t8329 / 144.0;
    let t8339 = t154 * t6446 * t1167;
    let t8340 = t385 * t8339;
    let t8342 = t1220 * t2344;
    let t8358 = t1235 * t2023;
    let t8359 = t8358 * t46;
    let t8360 = t2394 * t8359;
    let t8363 = t5939 * t1229;
    let t8364 = t918 * t8363;
    (t8325, t8329, t8331, t8339, t8340, t8342, t8358, t8359, t8360, t8363, t8364)
}
