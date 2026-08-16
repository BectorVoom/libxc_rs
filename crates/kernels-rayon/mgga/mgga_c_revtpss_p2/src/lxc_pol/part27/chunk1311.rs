//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1311/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1311(t13032: f64, t26848: f64, t13036: f64, t13040: f64, t7616: f64, t12959: f64, t26880: f64, t3650: f64, t7623: f64, t12881: f64, t7624: f64, t12854: f64, t29096: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97129 = t13032 * t26848;
    let t97133 = t13036 * t7616 * t13040;
    let t97136 = t26880 * t12959;
    let t97138 = t3650 * t7623;
    let t97141 = t7624 * t12881;
    let t97149 = t12854 * t29096;
    (t97129, t97133, t97136, t97138, t97141, t97149)
}
