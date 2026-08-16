//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2227/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2227(t16405: f64, t22833: f64, t16387: f64, t26309: f64, t16275: f64, t16271: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64, t26308: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91103 = t22833 * t16405;
    let t91105 = t26309 * t16387;
    let t91107 = t22833 * t16275;
    let t91109 = t22833 * t16271;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91114 = 7.0_f64 / 576.0_f64 * t91113;
    let t91116 = t3777 * t26308 * t5252;
    (t91103, t91105, t91107, t91109, t91114, t91116)
}
