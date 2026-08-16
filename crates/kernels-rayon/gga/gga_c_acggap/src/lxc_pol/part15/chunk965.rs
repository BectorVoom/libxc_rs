//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 965/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk965(t30811: f64, t4904: f64, t2450: f64, t7431: f64, t8461: f64, t8653: f64, t1988: f64, t8541: f64, t4908: f64, t4680: f64, t7493: f64, t8648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34158 = t30811 * t4904;
    let t34161 = t2450 * t7431 * t8461;
    let t34162 = t34161 * t8653;
    let t34170 = t1988 * t8541;
    let t34172 = t30811 * t4908;
    let t34175 = t7493 * t4680 * t8648;
    (t34158, t34161, t34162, t34170, t34172, t34175)
}
