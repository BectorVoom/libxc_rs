//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1184/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1184(t1558: f64, t257: f64, t119767: f64, t247: f64, t2749: f64, t119757: f64, t31846: f64, t4451: f64, t119752: f64, t120097: f64, t4367: f64, t119821: f64, t31753: f64, t4486: f64, t827: f64, t828: f64, t8478: f64) -> (f64, f64, f64, f64, f64) {
    let t126046 = t257 * t1558;
    let t126049 = t119767 * t247 * t126046 * t2749;
    let t126052 = t31846 * t119757 * t4451;
    let t126055 = t120097 * t119752 * t4367;
    let t126062 = t8478 * t119821 * t31753 * t827 * t828 * t4486;
    (t126046, t126049, t126052, t126055, t126062)
}
