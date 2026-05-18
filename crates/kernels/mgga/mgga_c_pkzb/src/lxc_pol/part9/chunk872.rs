//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 872/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk872<F: Float>(t6366: F, t6368: F, t2382: F, t2434: F, t2381: F, t1478: F, t154: F, t386: F, t385: F, t465: F, t931: F, t179: F, t824: F) -> (F, F, F, F, F, F, F) {
    let t6369 = t6366 * t6368;
    let t6372 = t2434 * t2382;
    let t6373 = t2381 * t6372;
    let t6377 = t154 * t1478 * t386;
    let t6379 = F::new(5.0) / F::new(1296.0) * t385 * t6377;
    let t6380 = t465 * t931;
    let t6382 = t179 * t6380 * t824;
    (t6369, t6372, t6373, t6377, t6379, t6380, t6382)
}
