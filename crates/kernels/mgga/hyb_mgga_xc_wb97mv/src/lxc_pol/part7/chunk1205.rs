//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1205/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1205<F: Float>(t10193: F, t3728: F, t7831: F, t516: F, t7917: F, t10544: F, t10906: F, t11578: F, t11580: F, t11649: F, t11650: F, t11651: F, t11653: F, t11655: F, t8518: F, t8882: F, t9246: F, t9611: F, sigma0: F) -> (F, F, F, F) {
    let t28860 = t3728 * t10193;
    let t28966 = t3728 * t7831;
    let t28974 = t516 * t7917 * sigma0;
    let t29023 = 2.0 * t10906 + 2.0 * t10544 + 2.0 * t8518 + 2.0 * t11655 + 4.0 * t11653 + 2.0 * t9611 + 4.0 * t11651 + 2.0 * t9246 + 2.0 * t8882 + 2.0 * t11650 + 2.0 * t11649 + 4.0 * t11578 + 2.0 * t11580;
    (t28860, t28966, t28974, t29023)
}
