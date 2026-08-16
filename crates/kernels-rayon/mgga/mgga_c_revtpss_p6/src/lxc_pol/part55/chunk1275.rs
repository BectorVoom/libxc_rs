//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1275/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1275(t10301: f64, t34409: f64, t2247: f64, t29362: f64, t8435: f64, t60224: f64, t8736: f64, t10309: f64, t13272: f64, t32801: f64, t122885: f64, t45972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t129165 = t10301 * t34409;
    let t129169 = t2247 * t8435 * t29362;
    let t129180 = t60224 * t8736;
    let t129193 = t10309 * t34409;
    let t129213 = t13272 * t32801;
    let t129216 = t45972 * t122885;
    (t129165, t129169, t129180, t129193, t129213, t129216)
}
