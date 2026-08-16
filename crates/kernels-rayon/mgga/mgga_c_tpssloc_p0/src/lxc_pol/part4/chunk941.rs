//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 941/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk941(t13602: f64, t1553: f64, t2403: f64, t4392: f64, t699: f64, t13550: f64, t13563: f64, t1543: f64, t2791: f64, t2970: f64, t4343: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13603 = 2.0_f64 / 9.0_f64 * t13602;
    let t13642 = t2403 * t1553;
    let t13644 = t699 * t4392;
    let t13645 = 0.10954222222222222222e0_f64 * t13644;
    let t13650 = 0.19931111111111111111e0_f64 * t13602;
    let t13675 = 0.22076e0_f64 * t13550;
    let t13679 = 0.13418888888888888889e0_f64 * t13563;
    let t13709 = 0.11038e0_f64 * t13644;
    let t13712 = 0.20128333333333333334e0_f64 * t13602;
    let t13727 = t1543 * t2791;
    let t13748 = t2970 * t4343;
    let t13750 = t973 * t13748 / 216.0_f64;
    (t13603, t13642, t13644, t13645, t13650, t13675, t13679, t13709, t13712, t13727, t13750)
}
