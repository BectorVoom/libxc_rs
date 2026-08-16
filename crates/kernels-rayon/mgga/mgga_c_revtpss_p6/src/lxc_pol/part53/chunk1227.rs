//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1227/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1227(t670: f64, t8740: f64, t28187: f64, t8764: f64, t34399: f64, t7316: f64, t32822: f64, t7901: f64, t28173: f64, t27060: f64, t7742: f64, t29432: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129431 = t8740 * t670;
    let t129436 = t8764 * t28187;
    let t129437 = t34399 * t7316;
    let t129438 = t32822 * t7901;
    let t129440 = t8764 * t28173;
    let t129445 = t27060 * t7742;
    let t129447 = t29432 * t7742;
    (t129431, t129436, t129437, t129438, t129440, t129445, t129447)
}
