//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 995/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk995<F: Float>(t1117: F, t2923: F, t9604: F, t9621: F, t9624: F, t9625: F, t9629: F, t9632: F, t9636: F, t9639: F, t9642: F, t9646: F, t9650: F, t9654: F, t9657: F, t9660: F, t9663: F, t9667: F, t9670: F) -> (F,) {
    let t9677 = 1400.0 / 3.0 * t9621 * t9604 - 180.0 * t9624 * t9625 * t2923 - 4.0 * t1117 * t9629 + 800.0 / 27.0 * t9632 * t9636 + 800.0 / 27.0 * t9639 * t9636 - 128.0 / 27.0 * t9642 * t9646 - 64.0 / 9.0 * t9632 * t9650 + 128.0 / 27.0 * t9654 * t9657 + 64.0 / 9.0 * t9639 * t9660 - 64.0 / 27.0 * t9663 * t9650 + 128.0 / 81.0 * t9667 * t9657 + 64.0 / 27.0 * t9670 * t9660 + 800.0 / 81.0 * t9663 * t9636 + 800.0 / 81.0 * t9670 * t9636;
    (t9677,)
}
