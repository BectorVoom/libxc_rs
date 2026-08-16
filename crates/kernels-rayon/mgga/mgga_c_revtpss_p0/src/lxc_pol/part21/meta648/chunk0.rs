//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2433/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2433(t11199: f64, t988: f64, t378: f64, t11198: f64, t340: f64, t338: f64, t11119: f64, t384: f64, t225: f64, t41306: f64, t3057: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42051 = t988 * t11199;
    let t42052 = t42051 * t378;
    let t42058 = 1.0_f64 / t11198 / t340;
    let t42059 = t338 * t42058;
    let t42060 = t42059 * t378;
    let t42066 = 1.0_f64 / t11119 / t384;
    let t42067 = t225 * t42066;
    let t42078 = 0.15365432098765432099e0_f64 * t41306;
    let t42107 = t3057 * t3259;
    (t42051, t42052, t42059, t42060, t42067, t42078, t42107)
}
