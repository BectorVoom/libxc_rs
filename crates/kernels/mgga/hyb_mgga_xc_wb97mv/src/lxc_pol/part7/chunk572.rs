//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 572/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk572<F: Float>(t1101: F, t2697: F, t2352: F, t2624: F, t2629: F, t2636: F, t2662: F, t2670: F, t2672: F, t2674: F, t2675: F, t2687: F, t2696: F, t1077: F, t484: F) -> (F, F, F, F) {
    let t2698 = t2697 * t1101;
    let t2700 = -t2352 - t2624 + t2629 - t2636 + t2662 + t2670 + t2672 - t2674 + 8.0 * t2675 - t2687 - t2696 - 0.11696447245269292414e1 * t2698;
    let t2701 = t1077 * t484;
    let t2702 = 1.0 / t2701;
    (t2698, t2700, t2701, t2702)
}
