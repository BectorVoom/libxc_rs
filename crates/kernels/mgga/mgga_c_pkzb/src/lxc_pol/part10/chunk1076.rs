//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1076/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1076<F: Float>(t3565: F, t702: F, t1096: F, t2815: F, t3581: F, t3578: F, t1940: F, t3577: F, t2819: F, t3564: F, t5873: F, t3592: F, t721: F, t1108: F, t2848: F, t3608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9410 = t3565 * t702;
    let t9413 = t1096 * t2815;
    let t9416 = t3581 * t702;
    let t9419 = t3578 * t702;
    let t9422 = t3577 * t1940;
    let t9423 = t9422 * t702;
    let t9426 = t2819 * t2815;
    let t9429 = t3564 * t5873;
    let t9430 = t9429 * t702;
    let t9437 = t3592 * t721;
    let t9440 = t1108 * t2848;
    let t9443 = t3608 * t721;
    (t9410, t9413, t9416, t9419, t9422, t9423, t9426, t9429, t9430, t9437, t9440, t9443)
}
