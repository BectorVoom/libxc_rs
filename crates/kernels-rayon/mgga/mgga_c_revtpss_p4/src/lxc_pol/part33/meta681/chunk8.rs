//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2229/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2229(t104115: f64, t109204: f64, t109222: f64, t109224: f64, t109226: f64, t109228: f64, t109230: f64, t111696: f64, t111734: f64, t1518: f64, t21881: f64, t27060: f64, t29427: f64, t29432: f64, t34446: f64, t4292: f64, t5920: f64, t670: f64, t7586: f64) -> f64 {
    let t111788 = 4.0_f64 * t104115 * t1518 + 2.0_f64 * t111696 * t670 + 4.0_f64 * t111734 * t1518 + 2.0_f64 * t21881 * t7586 + 2.0_f64 * t27060 * t5920 + 4.0_f64 * t29427 * t4292 + 2.0_f64 * t29432 * t5920 + 4.0_f64 * t34446 * t4292 + t109204 + t109222 + t109224 + t109226 + t109228 + t109230;
    t111788
}
