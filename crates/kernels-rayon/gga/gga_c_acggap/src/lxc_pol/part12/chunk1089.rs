//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1089/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1089(t1427: f64, t31491: f64, t7381: f64, t1345: f64, t1983: f64, t7380: f64, t1462: f64, t7614: f64, t1446: f64, t7605: f64, t1441: f64, t1456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35387 = t31491 * t7381 * t1427;
    let t35390 = t7380 * t1983 * t1345;
    let t35392 = t7614 * t1462;
    let t35394 = t7605 * t1446;
    let t35396 = t7605 * t1441;
    let t35398 = t7605 * t1456;
    (t35387, t35390, t35392, t35394, t35396, t35398)
}
