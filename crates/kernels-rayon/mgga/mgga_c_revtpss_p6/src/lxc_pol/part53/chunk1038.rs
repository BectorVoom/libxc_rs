//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1038/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1038(t117: f64, t32814: f64, t196: f64, t197: f64, t7687: f64, t2035: f64, t7313: f64, t8764: f64, t116: f64, t8740: f64) -> (f64, f64, f64, f64, f64) {
    let t32815 = t32814 * t117;
    let t32822 = t7687 * t196 * t197;
    let t32823 = t32822 * t2035;
    let t32824 = t8764 * t7313;
    let t32825 = t8740 * t116;
    (t32815, t32822, t32823, t32824, t32825)
}
