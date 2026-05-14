//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1073/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1073<F: Float>(t670: F, t8740: F, t28187: F, t8764: F, t34399: F, t7316: F, t32822: F, t7901: F, t28173: F, t27060: F, t7742: F, t29432: F, t28063: F, t7586: F, t651: F, t7002: F, t8233: F) -> (F, F, F, F, F, F, F, F, F) {
    let t129431 = t8740 * t670;
    let t129436 = t8764 * t28187;
    let t129437 = t34399 * t7316;
    let t129438 = t32822 * t7901;
    let t129440 = t8764 * t28173;
    let t129445 = t27060 * t7742;
    let t129447 = t29432 * t7742;
    let t129449 = t7586 * t28063;
    let t129452 = t651 * t8233 * t7002;
    (t129431, t129436, t129437, t129438, t129440, t129445, t129447, t129449, t129452)
}
