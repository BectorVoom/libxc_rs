//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1161/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1161<F: Float>(t28056: F, t7586: F, t104115: F, t1937: F, t111734: F, t29427: F, t6993: F, t28187: F, t8764: F, t34399: F, t7316: F, t32822: F, t7901: F) -> (F, F, F, F, F, F, F) {
    let t129407 = t7586 * t28056;
    let t129414 = t104115 * t1937;
    let t129416 = t111734 * t1937;
    let t129418 = t29427 * t6993;
    let t129436 = t8764 * t28187;
    let t129437 = t34399 * t7316;
    let t129438 = t32822 * t7901;
    (t129407, t129414, t129416, t129418, t129436, t129437, t129438)
}
