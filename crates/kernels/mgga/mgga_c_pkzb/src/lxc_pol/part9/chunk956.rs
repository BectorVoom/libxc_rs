//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 956/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk956<F: Float>(t1220: F, t2349: F, t154: F, t2347: F, t3026: F, t385: F, t7945: F, t907: F, t1167: F, t6446: F, t2344: F, t2387: F, t921: F, t2381: F, t2368: F, t2371: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8325 = t1220 * t2349 / 54.0;
    let t8329 = t154 * t2347 * t3026;
    let t8331 = t385 * t8329 / 144.0;
    let t8333 = t154 * t907 * t7945;
    let t8339 = t154 * t6446 * t1167;
    let t8340 = t385 * t8339;
    let t8342 = t1220 * t2344;
    let t8344 = t1167 * t2387;
    let t8345 = t8344 * t921;
    let t8346 = t2381 * t8345;
    let t8349 = t1167 * t2368;
    let t8350 = t8349 * t2371;
    (t8325, t8331, t8333, t8340, t8342, t8345, t8346, t8349, t8350)
}
