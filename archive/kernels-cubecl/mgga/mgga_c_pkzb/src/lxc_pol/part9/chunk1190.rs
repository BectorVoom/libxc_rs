//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1190/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1190<F: Float>(t1867: F, t667: F, t7375: F, t7378: F, t1281: F, t204: F, t2739: F) -> (F, F, F) {
    let t20707 = t1867 * t667;
    let t20708 = t7375 * t20707;
    let t20710 = t7378 * t20707;
    let t20716 = t204 * t1281 * t2739;
    (t20708, t20710, t20716)
}
