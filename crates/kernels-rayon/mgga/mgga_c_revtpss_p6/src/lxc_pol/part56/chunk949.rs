//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 949/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk949(t136: f64, t7565: f64, t2247: f64, t7574: f64, t8435: f64, t196: f64, t197: f64, t7687: f64, t2035: f64, t7313: f64, t8764: f64, t1936: f64, t27060: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32801 = t7565 * t136;
    let t32802 = t2247 * t32801;
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32822 = t7687 * t196 * t197;
    let t32823 = t32822 * t2035;
    let t32824 = t8764 * t7313;
    let t32828 = t27060 * t1936;
    (t32802, t32806, t32822, t32823, t32824, t32828)
}
