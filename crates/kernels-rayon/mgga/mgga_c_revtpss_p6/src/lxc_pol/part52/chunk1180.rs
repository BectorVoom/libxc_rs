//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1180/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1180(t119752: f64, t120097: f64, t4367: f64, t119821: f64, t31753: f64, t4486: f64, t827: f64, t828: f64, t8478: f64, t119757: f64, t31846: f64, t4461: f64) -> (f64, f64, f64) {
    let t126055 = t120097 * t119752 * t4367;
    let t126062 = t8478 * t119821 * t31753 * t827 * t828 * t4486;
    let t126065 = t31846 * t119757 * t4461;
    (t126055, t126062, t126065)
}
