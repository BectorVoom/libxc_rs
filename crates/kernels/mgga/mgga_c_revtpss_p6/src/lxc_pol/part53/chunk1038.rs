//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1038/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1038<F: Float>(t117: F, t32814: F, t196: F, t197: F, t7687: F, t2035: F, t7313: F, t8764: F, t116: F, t8740: F) -> (F, F, F, F, F) {
    let t32815 = t32814 * t117;
    let t32822 = t7687 * t196 * t197;
    let t32823 = t32822 * t2035;
    let t32824 = t8764 * t7313;
    let t32825 = t8740 * t116;
    (t32815, t32822, t32823, t32824, t32825)
}
