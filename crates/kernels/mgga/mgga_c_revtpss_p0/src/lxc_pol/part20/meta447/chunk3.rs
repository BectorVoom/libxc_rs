//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1709/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1709<F: Float>(t555: F, t9898: F, t14192: F, t2782: F, t9994: F, t544: F, t9989: F, t3923: F, t4003: F, t215: F, t268: F, t4056: F, t4101: F, t543: F) -> (F, F, F, F, F, F, F, F) {
    let t46469 = t555 * t9898;
    let t46472 = t2782 * t14192 * t46469 * t9994;
    let t46475 = F::new(1.0) / t9989 / t544;
    let t46476 = t46475 * t555;
    let t46477 = t3923 * t3923;
    let t46478 = t4003 * t4003;
    let t46479 = t46477 * t46478;
    let t46483 = t46477 * t9994;
    let t46490 = t4101 * t268 * t215 * t4056 * t543;
    (t46469, t46472, t46475, t46476, t46477, t46479, t46483, t46490)
}
