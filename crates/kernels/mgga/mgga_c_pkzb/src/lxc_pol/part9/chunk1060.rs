//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1060/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1060<F: Float>(t1034: F, t5373: F, t1721: F, t16399: F, t6908: F, t1702: F, t6930: F, t1769: F, t7005: F, t1734: F, t6859: F, t164: F, t1692: F, t1037: F, t16406: F, t16369: F, t6924: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20113 = t1034 * t5373;
    let t20114 = t20113 * t1721;
    let t20118 = t16399 * t6908;
    let t20121 = t1702 * t6930;
    let t20127 = t1769 * t7005;
    let t20137 = t6859 * t1734;
    let t20141 = t164 * t1692;
    let t20155 = t16406 * t1037;
    let t20157 = t16369 * t6924;
    (t20113, t20114, t20118, t20121, t20127, t20137, t20141, t20155, t20157)
}
