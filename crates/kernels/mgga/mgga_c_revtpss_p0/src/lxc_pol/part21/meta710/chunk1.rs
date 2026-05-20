//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2541/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541<F: Float>(t549: F, t240: F, t72: F, t4014: F, t9779: F, t221: F, t3978: F, t3979: F, t9628: F, t1408: F, t2237: F, t2482: F) -> (F, F, F, F) {
    let t46624 = t549 * t549;
    let t46625 = F::new(1.0) / t46624;
    let t46627 = t240 * t46625 * t72;
    let t46633 = t9779 * t4014;
    let t46641 = t3978 * t3979 * t221 * t9628;
    let t46644 = t2482 * t1408 * t2237;
    (t46627, t46633, t46641, t46644)
}
