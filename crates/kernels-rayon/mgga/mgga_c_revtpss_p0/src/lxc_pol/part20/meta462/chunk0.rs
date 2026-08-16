//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1756/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1756(t47187: f64, t543: f64, t820: f64, t843: f64, t9991: f64, t9997: f64, t1386: f64, t2237: f64, t2482: f64, t4021: f64, t235: f64, t46475: f64) -> (f64, f64, f64, f64) {
    let t47188 = t47187 * t543;
    let t47194 = t820 * t9991 * t843;
    let t47195 = t47194 * t9997;
    let t47198 = t2482 * t1386 * t2237;
    let t47199 = t47198 * t4021;
    let t47201 = t46475 * t235;
    (t47188, t47195, t47199, t47201)
}
