//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 989/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk989<F: Float>(t1531: F, t466: F, t5146: F, t4871: F, t4877: F, t1502: F, t1612: F, t16540: F, t4915: F, t555: F, t5137: F, t546: F, t4882: F, t513: F, t1511: F, t5089: F) -> (F, F, F, F, F, F, F) {
    let t16599 = 0.1301229756036208781e0 * t1531 * t466 * t5146;
    let t16600 = t4871 * t4877;
    let t16603 = 1.0 / t1502 / t1612;
    let t16607 = 0.12304822629859687989e5 * t555 * t16603 * t16540 * t4915;
    let t16612 = 480.0 * t5137 * t546;
    let t16613 = t4882 * t513;
    let t16615 = t1511 * t5089;
    (t16599, t16600, t16603, t16607, t16612, t16613, t16615)
}
