//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1922/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1922<F: Float>(t14874: F, t25270: F, t14746: F, t7025: F, t14769: F, t7045: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F, t14861: F) -> (F, F, F, F, F, F) {
    let t98993 = t25270 * t14874;
    let t98995 = t7025 * t14746;
    let t98997 = t7045 * t14769;
    let t99000 = t2661 * t25227 * t14727;
    let t99002 = t93034 * t4430;
    let t99006 = t2661 * t25227 * t14861;
    (t98993, t98995, t98997, t99000, t99002, t99006)
}
