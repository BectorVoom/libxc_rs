//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1296/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1296<F: Float>(t30792: F, t7616: F, t2147: F, t6398: F, t9292: F, t1616: F, t783: F, t9268: F, t6395: F, t9446: F, t30304: F, t538: F, t7623: F, t1634: F, t9327: F, t22856: F, t9126: F) -> (F, F, F, F, F, F, F) {
    let t30793 = t30792 * t7616;
    let t30796 = t2147 * t6398 * t9292;
    let t30801 = t783 * t9268 * t1616;
    let t30804 = t6395 * t9446;
    let t30807 = t7623 * t538 * t30304;
    let t30827 = t9327 * t1634;
    let t30840 = t22856 * t9126;
    (t30793, t30796, t30801, t30804, t30807, t30827, t30840)
}
