//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1374/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1374(t5: f64, t116759: f64, t116798: f64, t116821: f64, t116844: f64, t117: f64, t111696: f64, t114372: f64, t114375: f64, t114377: f64, t114380: f64, t114382: f64, t114384: f64, t114387: f64, t114389: f64, t114391: f64, t114403: f64, t116732: f64, t1518: f64, t22633: f64, t29427: f64, t34446: f64, t5920: f64, t7586: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t116847 = piecewise3(t8, 0.0_f64, t116759 + t116798 + t116821 + t116844);
    let t116848 = t116847 * t117;
    let t116861 = 6.0_f64 * t111696 * t1518 + 2.0_f64 * t22633 * t7586 + 6.0_f64 * t29427 * t5920 + 6.0_f64 * t34446 * t5920 + t114372 + t114375 + t114377 + t114380 + t114382 + t114384 + t114387 + t114389 + t114391 + t114403 + 6.0_f64 * t116732 + t116848;
    (t116848, t116861)
}
