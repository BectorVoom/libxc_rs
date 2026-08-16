//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2440/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2440<F: Float>(t1401: F, t46722: F, t159: F, t216: F, t4010: F, t1386: F, t2482: F, t2668: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F) -> (F, F, F, F, F) {
    let t46723 = t46722 * t1401;
    let t46730 = t216 * t159 * t4010;
    let t46740 = t2482 * t1386 * t2668;
    let t46760 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    (t46723, t46730, t46740, t46760, t46766)
}
