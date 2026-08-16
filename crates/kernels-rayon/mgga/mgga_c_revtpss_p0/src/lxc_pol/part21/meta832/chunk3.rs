//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3108/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3108(t12784: f64, t17451: f64, t17416: f64, t3647: f64, t11262: f64, t1247: f64, t5286: f64, t17501: f64, t3172: f64, t3711: f64, t13099: f64, t43776: f64) -> (f64, f64, f64, f64, f64) {
    let t57114 = t12784 * t17451;
    let t57118 = t3647 * t17416;
    let t57125 = t1247 * t11262 * t5286;
    let t57126 = 0.14291339372689912324e-3_f64 * t57125;
    let t57128 = t3711 * t3172 * t17501;
    let t57136 = t13099 * t43776;
    (t57114, t57118, t57126, t57128, t57136)
}
