//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1932/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1932(t25207: f64, t61102: f64, t14365: f64, t14436: f64, t18875: f64, t94245: f64, t25759: f64, t61203: f64, t98674: f64, t98759: f64, t98651: f64, t15071: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99558 = t25207 * t61102;
    let t100858 = t14436 * t14365;
    let t100944 = t94245 * t18875;
    let t100947 = t25759 * t61203;
    let t100953 = t25759 * t98674;
    let t100958 = t25759 * t98759;
    let t100964 = t25759 * t98651;
    let t100969 = t33 * t15071;
    (t99558, t100858, t100944, t100947, t100953, t100958, t100964, t100969)
}
