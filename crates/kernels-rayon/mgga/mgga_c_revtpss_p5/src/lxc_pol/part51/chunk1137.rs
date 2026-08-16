//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1137/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1137(t33703: f64, t689: f64, t120151: f64, t120005: f64, t33711: f64, t846: f64, t1568: f64, t31805: f64, t817: f64, t8485: f64, t31845: f64, t33695: f64) -> (f64, f64, f64, f64, f64) {
    let t126102 = t33703 * t689;
    let t126103 = t120151 * t126102;
    let t126105 = t120005 * t126102;
    let t126108 = t33711 * t846;
    let t126110 = t31805 * t1568;
    let t126112 = t126110 * t8485 * t817;
    let t126121 = t33695 * t31845;
    (t126103, t126105, t126108, t126112, t126121)
}
