//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 497/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk497(t213: f64, t218: f64, t978: f64, t2018: f64, t88: f64, t2014: f64, t215: f64, t982: f64, t2026: f64, t220: f64, t43: f64, t385: f64, t991: f64, t426: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2851 = t978 * t978;
    let t2855 = 2.0_f64 * t88 + 2.0_f64 * t2018;
    let t2859 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t2014 * t2851 + 4.0_f64 / 3.0_f64 * t215 * t2855);
    let t2860 = t982 * t982;
    let t2863 = -t2855;
    let t2867 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t2026 * t2860 + 4.0_f64 / 3.0_f64 * t220 * t2863);
    let t2869 = (t2859 + t2867) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    (t2869, t2874, t2876)
}
