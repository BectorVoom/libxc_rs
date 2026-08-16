//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2547/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2547<F: Float>(t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t4006: F, t1399: F, t2661: F, t3992: F, t9929: F, t1412: F, t3938: F, t4056: F) -> (F, F, F, F) {
    let t46760 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    let t46767 = t46766 * t4006;
    let t46771 = t2661 * t3992 * t9929 * t1399;
    let t46776 = t2661 * t3992 * t1412 * t4056 * t3938;
    (t46760, t46767, t46771, t46776)
}
