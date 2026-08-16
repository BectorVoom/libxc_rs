//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2049/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2049(t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t25240: f64, t3964: f64, t5617: f64, t27857: f64, t689: f64, t25904: f64, t786: f64, t97961: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98269 = t26004 * t5690;
    let t98270 = 7.0_f64 / 72.0_f64 * t98269;
    let t98281 = t807 * t2018 * t13951;
    let t98282 = 0.11433071498151929859e-3_f64 * t98281;
    let t98285 = t3964 * t25240 * t5617;
    let t98303 = t27857 * t689;
    let t98305 = 0.14456046980341999104e-1_f64 * t25904 * t98303;
    let t98308 = t786 * t97961;
    (t98270, t98282, t98285, t98303, t98305, t98308)
}
