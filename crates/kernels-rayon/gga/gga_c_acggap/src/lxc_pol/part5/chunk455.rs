//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 455/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk455(t43: f64, t50: f64, t560: f64, t1690: f64, t1694: f64, t292: f64, t817: f64, t1699: f64, t1702: f64, t296: f64, t829: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1717 = t560 * t560;
    let t1726 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t817 * t1690 + 2.0_f64 / 3.0_f64 * t292 * t1694);
    let t1732 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t829 * t1699 + 2.0_f64 / 3.0_f64 * t296 * t1702);
    let t1734 = t1726 / 2.0_f64 + t1732 / 2.0_f64;
    (t1717, t1734)
}
