//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1164/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1164(t7575: f64, t8480: f64, t8514: f64, t2288: f64, t8791: f64, t13287: f64, t34823: f64, t1181: f64, t2068: f64, t38784: f64, t599: f64, t1165: f64, t39743: f64, t604: f64, t7346: f64) -> (f64, f64, f64, f64, f64) {
    let t40063 = t7575 * t8480 * t8514;
    let t40066 = t2288 * t8791;
    let t40068 = t34823 * t13287 * t40066;
    let t40072 = t2068 * t1181 * t599 * t38784;
    let t40076 = t7346 * t1165 * t604 * t39743;
    (t40063, t40066, t40068, t40072, t40076)
}
