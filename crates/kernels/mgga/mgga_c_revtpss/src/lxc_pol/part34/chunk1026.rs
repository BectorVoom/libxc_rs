//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1026/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1026<F: Float>(t25515: F, t4890: F, t3299: F, t3317: F, t1967: F, t816: F, t1014: F, t65: F, t3252: F, t3204: F, t7131: F, t4817: F, t7132: F, t7810: F, t994: F, t1976: F, t4746: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27498 = t3317 * t27492;
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    let t27531 = t65 * t3252;
    let t27536 = t3204 * t7131;
    let t27539 = t7132 * t4817;
    let t27550 = t994 * t7810;
    let t27568 = t4746 * t1976;
    (t27492, t27493, t27498, t27526, t27527, t27531, t27536, t27539, t27550, t27568)
}
