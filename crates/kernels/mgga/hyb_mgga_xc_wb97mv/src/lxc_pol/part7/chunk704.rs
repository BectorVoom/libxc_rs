//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 704/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk704<F: Float>(t10: F, t1507: F, t1096: F, t2629: F, t2636: F, t2662: F, t2670: F, t2674: F, t2675: F, t2687: F, t3469: F, t3630: F, t3633: F, t3635: F, t3639: F, t3658: F, t489: F) -> (F, F, F) {
    let t3661 = t1507 * t10;
    let t3662 = t3661 * t1096;
    let t3664 = -t3630 + 4.0 * t3633 - 4.0 * t3635 + t2629 - t3469 - t2636 + t2662 + t2670 - t2674 - 4.0 * t2675 - 0.5848223622634646207e0 * t3639 + 0.19751673498613801407e-1 * t3658 * t489 - 0.18311447306006545054e-3 * t3662 - t2687;
    (t3661, t3662, t3664)
}
