//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 845/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk845<F: Float>(t10301: F, t8736: F, t10309: F, t136: F, t7565: F, t2247: F, t7574: F, t8435: F, t196: F, t197: F, t7687: F, t2035: F, t7313: F, t8764: F, t1936: F, t27060: F) -> (F, F, F, F, F, F, F, F) {
    let t32795 = t10301 * t8736;
    let t32798 = t10309 * t8736;
    let t32801 = t7565 * t136;
    let t32802 = t2247 * t32801;
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32822 = t7687 * t196 * t197;
    let t32823 = t32822 * t2035;
    let t32824 = t8764 * t7313;
    let t32828 = t27060 * t1936;
    (t32795, t32798, t32802, t32806, t32822, t32823, t32824, t32828)
}
