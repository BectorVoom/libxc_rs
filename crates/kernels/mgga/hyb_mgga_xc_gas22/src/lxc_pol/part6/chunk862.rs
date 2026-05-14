//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 862/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk862<F: Float>(t16: F, t3021: F, t492: F, t1105: F, t2699: F, t2707: F, t1052: F, t2742: F, t1057: F, t2747: F, t496: F, t5891: F, t1056: F, t457: F, t1051: F, t2750: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7520 = t16 * t3021 * t492;
    let t7522 = 0.56968947174242584612e-3 * t1105 * t7520;
    let t7523 = t2699 * t2707;
    let t7526 = t1052 * t2742;
    let t7528 = t1057 * t2742;
    let t7530 = t1052 * t2747;
    let t7532 = t1057 * t2747;
    let t7535 = 24.0 * t5891 * t496;
    let t7536 = t457 * t1056;
    let t7537 = t7536 * t496;
    let t7539 = t1051 * t2750;
    (t7520, t7522, t7523, t7526, t7528, t7530, t7532, t7535, t7536, t7537, t7539)
}
