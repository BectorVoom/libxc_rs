//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1303/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1303<F: Float>(t2563: F, t3433: F, t20338: F, t129: F, t524: F, t6238: F, t2593: F, t277: F, t7088: F, t7494: F, t7509: F, t113: F, t4933: F, t20643: F, t259: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t24573 = t3433 * t2563;
    let t24574 = t20338 * t24573;
    let t24575 = 0.19043987679069580388e-1 * t24574;
    let t24581 = t524 * t6238 * t129;
    let t24582 = t24581 * t2593;
    let t24583 = 0.12805040077930161442e1 * t24582;
    let t24589 = t277 * t7088;
    let t24600 = t7494 * t7509;
    let t24609 = t113 * t4933;
    let t24615 = t571 * t20643 * t259;
    (t24575, t24581, t24583, t24589, t24600, t24609, t24615)
}
