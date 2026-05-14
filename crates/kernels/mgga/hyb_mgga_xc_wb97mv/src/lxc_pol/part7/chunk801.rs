//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 801/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk801<F: Float>(t2629: F, t2636: F, t2662: F, t2670: F, t2672: F, t2674: F, t3633: F, t3635: F, t3639: F, t4271: F, t4401: F, t4510: F, t458: F, t2687: F, t2696: F, t2706: F, t2711: F, t2714: F, t2717: F, t2720: F, t2756: F, t2777: F, t2784: F, t3662: F, t3669: F, t4509: F, t489: F) -> (F, F) {
    let t4513 = -t4401 - t4271 - 8.0 * t3633 - 8.0 * t3635 + t2629 + t458 * t4510 - t2636 + t2662 + t2670 - t2672 - t2674 - 0.11696447245269292414e1 * t3639;
    let t4518 = -0.36622894612013090108e-3 * t3662 - t2687 - t2696 + t2706 + t2711 + t2714 + t2717 - t2720 + t2777 + 2.0 * t3669 - t2756 + t2784 + 0.19751673498613801407e-1 * t4509 * t489;
    (t4513, t4518)
}
