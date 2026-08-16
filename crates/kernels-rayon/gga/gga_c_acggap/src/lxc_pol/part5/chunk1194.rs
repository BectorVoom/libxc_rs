//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1194/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1194(t1049: f64, t5663: f64, t1765: f64, t3143: f64, t1055: f64, t20092: f64, t345: f64, t1769: f64, t19510: f64, t346: f64, t13696: f64, t13699: f64, t13701: f64, t13706: f64, t13714: f64, t13727: f64, t13729: f64, t13737: f64, t16230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21707 = t1049 * t5663;
    let t21709 = t3143 * t1765;
    let t21712 = t345 * t1055 * t20092;
    let t21714 = t3143 * t1769;
    let t21717 = t345 * t346 * t19510;
    let t21720 = -0.21733333333333333334e1_f64 * t13696 + 0.1956e1_f64 * t13699 + 0.2445e0_f64 * t13701 + 0.2445e0_f64 * t13706 - 0.12225e0_f64 * t13714 + t13727 - 0.489e0_f64 * t13729 + t13737 + 0.978e0_f64 * t21707 + 0.2282e1_f64 * t21709 + 0.1467e1_f64 * t21712 - 0.1141e1_f64 * t21714 - 0.36675e0_f64 * t21717 + 0.978e0_f64 * t16230;
    (t21707, t21709, t21712, t21714, t21717, t21720)
}
