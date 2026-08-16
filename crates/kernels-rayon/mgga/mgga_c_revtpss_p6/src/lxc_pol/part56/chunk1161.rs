//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1161/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1161(t28056: f64, t7586: f64, t104115: f64, t1937: f64, t111734: f64, t29427: f64, t6993: f64, t28187: f64, t8764: f64, t34399: f64, t7316: f64, t32822: f64, t7901: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129407 = t7586 * t28056;
    let t129414 = t104115 * t1937;
    let t129416 = t111734 * t1937;
    let t129418 = t29427 * t6993;
    let t129436 = t8764 * t28187;
    let t129437 = t34399 * t7316;
    let t129438 = t32822 * t7901;
    (t129407, t129414, t129416, t129418, t129436, t129437, t129438)
}
