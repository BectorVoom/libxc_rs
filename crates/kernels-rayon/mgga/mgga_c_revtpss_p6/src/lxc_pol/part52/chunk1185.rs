//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1185/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1185(t31753: f64, t4435: f64, t8478: f64, t8484: f64, t817: f64, t8485: f64, t98848: f64, t126078: f64, t2747: f64, t31767: f64, t31772: f64, t124: f64, t1579: f64, t800: f64, t815: f64) -> (f64, f64, f64, f64) {
    let t126145 = t8478 * t8484 * t31753 * t4435;
    let t126148 = t98848 * t8485 * t817;
    let t126158 = t31767 * t2747 * t31772 * t126078;
    let t126163 = t815 * t800 * t124 * t1579;
    (t126145, t126148, t126158, t126163)
}
