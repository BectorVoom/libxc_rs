//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2488/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488<F: Float>(t13951: F, t2713: F, t3964: F, t1413: F, t46835: F, t48698: F, t1873: F, t46651: F, t13910: F, t808: F, t9736: F, t550: F, t9794: F) -> (F, F, F, F, F) {
    let t49008 = t3964 * t2713 * t13951;
    let t49012 = t46835 * t1413 * t48698;
    let t49030 = t46651 * t1873;
    let t49056 = t9736 * t808 * t13910;
    let t49057 = F::cast_from(0.30492001685571196935e-4_f64) * t49056;
    let t49068 = t9794 * t550;
    (t49008, t49012, t49030, t49057, t49068)
}
