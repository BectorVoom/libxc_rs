//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1335/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1335<F: Float>(t1145: F, t2876: F, t4544: F, t4540: F, t2884: F, t4535: F, t30771: F, t9561: F, t26728: F, t2869: F, t30596: F, t30670: F, t30772: F, t30784: F, t30860: F, t3720: F, t3739: F, t3747: F, t7734: F, t7739: F, t7769: F, t7775: F, t7780: F, t9527: F, t9594: F, t9636: F) -> (F, F, F) {
    let t31410 = t1145 * t4544 * t2876;
    let t31414 = t1145 * t4540 * t2876;
    let t31419 = t2884 * t4535;
    let t31436 = t9561 * t30771;
    let t31441 = -168.0 * t7780 * t31410 + 6.0 * t7739 * t31414 - 12.0 * t7769 * t31410 - 6400.0 / 27.0 * t3720 * t31419 + 60.0 * t7775 * t1145 * t4540 * t2869 + 126.0 * t7734 * t31414 - 3200.0 / 9.0 * t26728 * t30596 + 3200.0 / 9.0 * t30670 * t9636 - 16.0 / 3.0 * t9527 * t30784 + 5632.0 / 2187.0 * t9594 * t30772 + 1408.0 / 243.0 * t3739 * t31436 + 704.0 / 81.0 * t3747 * t30860;
    (t31419, t31436, t31441)
}
