//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1718/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1718<F: Float>(t3995: F, t40488: F, t3989: F, t9944: F, t549: F, t240: F, t72: F, t3829: F, t4014: F, t9779: F, t221: F, t3978: F, t3979: F, t9628: F) -> (F, F, F, F, F, F) {
    let t46620 = t40488 * t3995;
    let t46622 = t3989 * t9944;
    let t46624 = t549 * t549;
    let t46625 = F::new(1.0) / t46624;
    let t46627 = t240 * t46625 * t72;
    let t46628 = t3829 * t3829;
    let t46633 = t9779 * t4014;
    let t46641 = t3978 * t3979 * t221 * t9628;
    (t46620, t46622, t46627, t46628, t46633, t46641)
}
