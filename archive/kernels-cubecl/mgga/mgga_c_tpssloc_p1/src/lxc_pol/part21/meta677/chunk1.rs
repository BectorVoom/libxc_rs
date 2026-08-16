//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2484/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484<F: Float>(t12908: F, t12924: F, t4101: F, t9912: F, t1409: F, t2516: F, t4194: F, t607: F, t4199: F, t9722: F, t12887: F, t172: F, t763: F) -> (F, F, F, F, F) {
    let t46283 = t12908 * t12924;
    let t46285 = t9912 * t4101;
    let t46291 = t4194 * t2516 * t1409 * t607;
    let t46302 = t4199 * t9722;
    let t46308 = t12887 * t172 * t763;
    (t46283, t46285, t46291, t46302, t46308)
}
