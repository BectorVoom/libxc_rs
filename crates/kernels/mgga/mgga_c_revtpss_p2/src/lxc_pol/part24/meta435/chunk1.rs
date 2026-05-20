//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1388/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388<F: Float>(t1386: F, t2482: F, t2668: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t10111: F, t1408: F, t9720: F, t40735: F, t535: F) -> (F, F, F, F, F) {
    let t46740 = t2482 * t1386 * t2668;
    let t46760 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    let t46784 = t10111 * t1408 * t9720;
    let t46800 = F::new(455.0) / F::new(243.0) * t40735 * t535;
    (t46740, t46760, t46766, t46784, t46800)
}
