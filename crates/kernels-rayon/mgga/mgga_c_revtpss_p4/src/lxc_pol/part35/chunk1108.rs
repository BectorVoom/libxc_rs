//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1108/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1108(t221: f64, t23279: f64, t22648: f64, t602: f64, t1882: f64, t6843: f64, t22852: f64, t13790: f64, t543: f64, t23087: f64, t47672: f64, t23059: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76613 = t221 * t23279;
    let t85037 = t22648 * t602;
    let t85659 = t6843 * t1882;
    let t85776 = t221 * t22852;
    let t86413 = t13790 * t6843;
    let t86641 = t85659 * t543;
    let t86791 = t23087 * t47672;
    let t86825 = t23059 * t4147;
    (t76613, t85037, t85776, t86413, t86641, t86791, t86825)
}
