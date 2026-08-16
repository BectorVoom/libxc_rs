//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 499/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk499(t213: f64, t218: f64, t2014: f64, t978: f64, t211: f64, t215: f64, t414: f64, t690: f64, t2026: f64, t982: f64, t220: f64, t694: f64, t43: f64, t385: f64, t991: f64, zeta_threshold: f64) -> (f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2828 = t2014 * t978;
    let t2831 = t215 * t211;
    let t2835 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t2828 * t690 + 8.0_f64 / 3.0_f64 * t2831 * t414);
    let t2836 = t2026 * t982;
    let t2839 = t220 * t211;
    let t2843 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t2836 * t694 - 8.0_f64 / 3.0_f64 * t2839 * t414);
    let t2845 = (t2835 + t2843) * t43;
    let t2874 = t385 * t991;
    (t2845, t2874)
}
