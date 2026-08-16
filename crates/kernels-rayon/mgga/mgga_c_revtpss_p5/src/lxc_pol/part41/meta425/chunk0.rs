//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1484/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1484(t31032: f64, t31284: f64, t116912: f64, t31261: f64, t10208: f64, t69: f64, t96: f64, t100: f64, t1513: f64, t2339: f64, t31027: f64, t31268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117484 = 20.0_f64 / 27.0_f64 * t31032 * t31284;
    let t117497 = 4.0_f64 * t116912 * t31261;
    let t117499 = t69 * t10208 * t96;
    let t117500 = t100 * t1513;
    let t117505 = t69 * t2339 * t96;
    let t117510 = 20.0_f64 / 9.0_f64 * t31027 * t31268;
    (t117484, t117497, t117499, t117500, t117505, t117510)
}
