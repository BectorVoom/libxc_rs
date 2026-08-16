//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3155/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155(t127: f64, t12866: f64, t17650: f64, t5296: f64, t17861: f64, t3624: f64, t12784: f64, t17451: f64, t17416: f64, t3647: f64, t11262: f64, t1247: f64, t5286: f64) -> (f64, f64, f64, f64, f64) {
    let t57098 = t12866 * t127 * t5296 * t17650;
    let t57100 = t17861 * t3624;
    let t57114 = t12784 * t17451;
    let t57118 = t3647 * t17416;
    let t57125 = t1247 * t11262 * t5286;
    (t57098, t57100, t57114, t57118, t57125)
}
