//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1001/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1001<F: Float>(t22674: F, t33296: F, t6897: F, t22751: F, t33307: F, t1985: F, t8621: F, t90739: F, t115545: F, t1992: F, t26355: F, t22633: F, t22635: F, t31549: F, t5187: F) -> (F, F, F, F, F) {
    let t122247 = t6897 * t22674 * t33296;
    let t122251 = t22751 * t33307;
    let t122260 = t1985 * t90739 * t8621;
    let t122270 = t1992 * t115545 * t26355;
    let t122278 = t22633 * t22635 * t31549 * t5187;
    (t122247, t122251, t122260, t122270, t122278)
}
