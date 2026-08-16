//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 993/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk993(t12621: f64, t1280: f64, t3634: f64, t828: f64, t3630: f64, t3625: f64, t3372: f64, t5405: f64, t3626: f64, t3368: f64, t3624: f64, t3746: f64) -> (f64, f64, f64, f64, f64) {
    let t12769 = t1280 * t12621;
    let t12772 = t828 * t3634;
    let t12773 = t12772 * t3630;
    let t12774 = t3625 * t12773;
    let t12776 = t3372 * t5405;
    let t12777 = t3626 * t12776;
    let t12780 = t3368 * t5405;
    let t12781 = t3626 * t12780;
    let t12784 = t3746 * t3624;
    (t12769, t12774, t12777, t12781, t12784)
}
