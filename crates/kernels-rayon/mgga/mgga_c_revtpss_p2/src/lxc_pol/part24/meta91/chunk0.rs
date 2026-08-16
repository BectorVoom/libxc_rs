//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 531/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk531(t2470: f64, t874: f64, t875: f64, t251: f64, t2718: f64, t1941: f64, t268: f64, t271: f64, t1065: f64, t159: f64) -> (f64, f64, f64, f64, f64) {
    let t2810 = 0.13009920719177044025e-1_f64 * t874 * t875 * t2470;
    let t2811 = t2718 * t251;
    let t2846 = t268 * t1941 * t271;
    let t2847 = 0.23744444444444444444e-1_f64 * t2846;
    let t2850 = t159 * t1065;
    (t2810, t2811, t2846, t2847, t2850)
}
