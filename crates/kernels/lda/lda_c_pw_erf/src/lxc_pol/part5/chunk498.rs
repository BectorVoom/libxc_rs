//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 498/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk498<F: Float>(t1325: F, t2558: F, t1334: F, t2334: F, t574: F, t571: F, t1339: F, t2325: F, t522: F, t519: F, t1938: F, t1985: F, t2266: F, t2538: F, t2542: F, t2546: F, t2548: F, t2552: F, t2556: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2560 = 8.0 / 15.0 * t1325 * t2558;
    let t2561 = t1334 * t2334;
    let t2562 = t574 * t2561;
    let t2564 = 8.0 / 45.0 * t571 * t2562;
    let t2565 = t1339 * t2325;
    let t2566 = t522 * t2565;
    let t2568 = 8.0 / 45.0 * t519 * t2566;
    let t2569 = 8.0 / 45.0 * t1938;
    let t2570 = 8.0 / 45.0 * t1985;
    let t2572 = t2538 + t2542 + t2546 + t2548 + t2552 + t2556 - t2560 - t2564 - t2568 + t2569 + t2570 - 4.0 / 45.0 * t2266;
    (t2560, t2561, t2562, t2564, t2565, t2566, t2568, t2569, t2570, t2572)
}
