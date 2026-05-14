//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 177/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk177<F: Float>(t546: F, t83: F, t124: F, t512: F, t46: F, t99: F, t123: F, t465: F, t475: F) -> (F, F, F, F) {
    let t547 = t83 * t546;
    let t549 = 0.19751673498613801407e-1 * t512 * t124;
    let t550 = t99 * t46;
    let t552 = t475 * t465 * t123;
    (t547, t549, t550, t552)
}
