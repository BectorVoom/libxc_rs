//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 734/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk734(t3017: f64, t5017: f64, t8737: f64, t3022: f64, t3028: f64, t1033: f64, t5486: f64, t169: f64, t474: f64, t619: f64, t116: f64, t5463: f64) -> (f64, f64, f64, f64, f64) {
    let t8738 = t3017 * t5017;
    let t8739 = t8737 * t8738;
    let t8741 = t3028 * t3022;
    let t8743 = t5486 * t1033;
    let t8744 = t169 * t8743;
    let t8745 = t474 * t619;
    let t8746 = t8744 * t8745;
    let t8748 = t116 * t5463;
    (t8738, t8739, t8741, t8746, t8748)
}
