//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1142/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1142(t7252: f64, t9700: f64, t64: f64, t9990: f64, t239: f64, t820: f64, t9997: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25986: f64, t2661: f64, t9980: f64) -> (f64, f64, f64, f64) {
    let t94487 = t7252 * t9700;
    let t94491 = t9990 * t64;
    let t94493 = t820 * t94491 * t239;
    let t94494 = t94493 * t9997;
    let t94497 = t2482 * t7262 * t596;
    let t94498 = t94497 * t4021;
    let t94501 = t2661 * t25986 * t9980;
    (t94487, t94494, t94498, t94501)
}
