//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1102/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1102(t136: f64, t243: f64, t3133: f64, t3302: f64, t357: f64, t2371: f64, t94: f64, t4982: f64, t999: f64, t2007: f64, t197: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14685 = t243 * t136;
    let t16573 = t3302 * t3133 * t357;
    let t18163 = t94 * t2371;
    let t19482 = t3302 * t357;
    let t19502 = t4982 * t999;
    let t19579 = t19482 * t999;
    let t25078 = t2007 * t2371;
    let t25081 = t197 * t531;
    (t14685, t16573, t18163, t19482, t19502, t19579, t25078, t25081)
}
