//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1922/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1922(t14874: f64, t25270: f64, t14746: f64, t7025: f64, t14769: f64, t7045: f64, t14727: f64, t25227: f64, t2661: f64, t4430: f64, t93034: f64, t14861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98993 = t25270 * t14874;
    let t98995 = t7025 * t14746;
    let t98997 = t7045 * t14769;
    let t99000 = t2661 * t25227 * t14727;
    let t99002 = t93034 * t4430;
    let t99006 = t2661 * t25227 * t14861;
    (t98993, t98995, t98997, t99000, t99002, t99006)
}
