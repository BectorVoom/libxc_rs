//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1084/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1084<F: Float>(t2105: F, t9558: F, t2029: F, t3650: F, t2901: F, t302: F, t2923: F, t2976: F, t3645: F, t2739: F, t287: F, t1137: F, t154: F, t3542: F, t5663: F, t276: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9559 = t2105 * t9558;
    let t9562 = t3650 * t2029;
    let t9563 = t9562 * t2901;
    let t9564 = t302 * t9563;
    let t9567 = t9562 * t2923;
    let t9568 = t302 * t9567;
    let t9571 = t2976 * t3645;
    let t9572 = t2105 * t9571;
    let t9575 = t287 * t2739;
    let t9576 = t1137 * t9575;
    let t9577 = t2105 * t9576;
    let t9583 = t154 * t5663 * t3542;
    let t9584 = t276 * t9583;
    (t9559, t9562, t9563, t9564, t9567, t9568, t9571, t9572, t9575, t9576, t9577, t9583, t9584)
}
