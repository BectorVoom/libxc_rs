//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 490/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk490(t213: f64, t218: f64, t2551: f64, t2653: f64, t2740: f64, t2820: f64, t978: f64, t2018: f64, t88: f64, t2014: f64, t215: f64, t982: f64, t2026: f64, t220: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2822 = t2551 + t2653 + t2740 + t2820;
    let t2851 = t978 * t978;
    let t2855 = 2.0_f64 * t88 + 2.0_f64 * t2018;
    let t2859 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t2014 * t2851 + 4.0_f64 / 3.0_f64 * t215 * t2855);
    let t2860 = t982 * t982;
    let t2863 = -t2855;
    let t2867 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t2026 * t2860 + 4.0_f64 / 3.0_f64 * t220 * t2863);
    (t2822, t2859, t2867)
}
