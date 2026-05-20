//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1405/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405<F: Float>(t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t14202: F, t9303: F, t14238: F, t2453: F, t10139: F, t14219: F, t9285: F) -> (F, F, F, F, F) {
    let t47967 = t46389 * t5735 * t543 * t22;
    let t47971 = t1432 * t5763 * t9288;
    let t48005 = t9303 * t14202;
    let t48007 = t2453 * t14238;
    let t48036 = t10139 * t14219 * t9285;
    (t47967, t47971, t48005, t48007, t48036)
}
