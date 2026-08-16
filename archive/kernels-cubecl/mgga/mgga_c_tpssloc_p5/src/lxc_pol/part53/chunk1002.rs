//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1002/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1002<F: Float>(t33272: F, t81228: F, t81326: F, t33240: F, t6883: F, t115545: F, t26331: F, t26333: F, t26189: F, t31611: F, t6888: F, t115352: F, t22892: F, t7691: F) -> (F, F, F, F, F) {
    let t122281 = t81228 * t81326 * t33272;
    let t122295 = t6883 * t33240;
    let t122304 = t26331 * t115545 * t26333;
    let t122328 = t6888 * t31611 * t26189;
    let t122331 = t22892 * t115352 * t7691;
    (t122281, t122295, t122304, t122328, t122331)
}
