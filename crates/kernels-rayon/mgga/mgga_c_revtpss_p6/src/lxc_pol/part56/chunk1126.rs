//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1126/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1126(t120016: f64, t126133: f64, t1544: f64, t886: f64, t119792: f64, t828: f64, t855: f64, t31753: f64, t4435: f64, t8478: f64, t8484: f64, t817: f64, t8485: f64, t98848: f64) -> (f64, f64, f64, f64, f64) {
    let t126136 = t120016 * t126133;
    let t126138 = t1544 * t886;
    let t126141 = t119792 * t855 * t828 * t126138;
    let t126145 = t8478 * t8484 * t31753 * t4435;
    let t126148 = t98848 * t8485 * t817;
    (t126136, t126138, t126141, t126145, t126148)
}
