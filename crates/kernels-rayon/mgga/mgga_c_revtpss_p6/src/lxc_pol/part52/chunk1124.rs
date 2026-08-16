//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1124/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1124(t119967: f64, t120010: f64, t119837: f64, t14686: f64, t837: f64, t119833: f64, t814: f64, t853: f64, t802: f64, t31827: f64, t844: f64, t31853: f64, t8486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120011 = t119967 * t120010;
    let t120013 = t14686 * t119837 * t837;
    let t120014 = t120011 * t120013;
    let t120016 = t119833 * t120010;
    let t120017 = t120016 * t120013;
    let t120042 = t814 * t853;
    let t120043 = t120042 * t802;
    let t120044 = t31827 * t120043;
    let t120045 = 0.14874931683620404328e-3_f64 * t120044;
    let t120046 = t844 * t853;
    let t120048 = t8486 * t120046 * t31853;
    (t120011, t120014, t120016, t120017, t120042, t120043, t120045, t120046, t120048)
}
