//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1172/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1172(t670: f64, t8916: f64, t124533: f64, t125531: f64, t125536: f64, t125558: f64, t125562: f64, t129354: f64, t129357: f64, t129366: f64, t129371: f64, t129376: f64, t129378: f64, t129395: f64, t1519: f64, t27060: f64, t29432: f64, t29444: f64, t29456: f64, t33346: f64, t4257: f64, t7586: f64, t8158: f64) -> (f64, f64) {
    let t131338 = t8916 * t670;
    let t131356 = -2.0_f64 * t124533 * t1519 - 2.0_f64 * t131338 * t1519 - 4.0_f64 * t27060 * t8158 - 4.0_f64 * t29432 * t8158 - 4.0_f64 * t29444 * t7586 - 4.0_f64 * t29456 * t7586 - 2.0_f64 * t33346 * t4257 - t125531 + t125536 + t125558 - t125562 + 4.0_f64 * t129354 - 4.0_f64 * t129357 - 6.0_f64 * t129366 + 2.0_f64 * t129371 + 2.0_f64 * t129376 + 12.0_f64 * t129378 - 4.0_f64 * t129395;
    (t131338, t131356)
}
