//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 494/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk494(t218: f64, t211: f64, t220: f64, t2836: f64, t414: f64, t694: f64, t2835: f64, t43: f64, t385: f64, t991: f64, t426: f64, t118: f64, t632: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t219 = t218 <= zeta_threshold;
    let t2839 = t220 * t211;
    let t2843 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t2836 * t694 - 8.0_f64 / 3.0_f64 * t2839 * t414);
    let t2845 = (t2835 + t2843) * t43;
    let t2874 = t385 * t991;
    let t2876 = t426 * t991;
    let t2878 = t632 * t118;
    (t2845, t2874, t2876, t2878)
}
