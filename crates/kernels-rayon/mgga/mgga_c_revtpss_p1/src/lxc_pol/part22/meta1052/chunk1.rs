//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3714/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3714(t20810: f64, t3172: f64, t3711: f64, t17412: f64, t5378: f64, t17416: f64, t5381: f64, t12915: f64, t20721: f64, t247: f64, t5384: f64, t1214: f64, t21082: f64) -> (f64, f64, f64, f64, f64) {
    let t70394 = t3711 * t3172 * t20810;
    let t70403 = t17412 * t5378;
    let t70405 = t5381 * t17416;
    let t70411 = t5384 * t247 * t12915 * t20721;
    let t70413 = t21082 * t1214;
    (t70394, t70403, t70405, t70411, t70413)
}
