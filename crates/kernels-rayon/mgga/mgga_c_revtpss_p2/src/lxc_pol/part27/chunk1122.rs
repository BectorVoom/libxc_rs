//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1122/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1122(t114: f64, t25821: f64, t624: f64, t655: f64, t665: f64, t2339: f64, t68: f64, t2340: f64, t2366: f64, t6998: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t25822 = 11.0_f64 / 9.0_f64 * t25821;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    let t25825 = 2.0_f64 / 3.0_f64 * t25824;
    let t25826 = t68 * t2339;
    let t25827 = t25826 * t2340;
    let t25829 = t6998 * t2366;
    let t25832 = piecewise3(t115, 0.0_f64, t25822 + t25825 + t25827 / 4.0_f64 - t25829 / 8.0_f64);
    (t25823, t25826, t25832)
}
