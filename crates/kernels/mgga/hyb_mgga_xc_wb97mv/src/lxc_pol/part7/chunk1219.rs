//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1219/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1219<F: Float>(t3085: F, t3104: F, t10434: F, t10448: F, t10478: F, t10495: F, t1924: F, t1937: F, t1945: F, t21600: F, t25201: F, t29466: F, t29484: F, t3103: F, t3113: F, t3926: F, t3931: F, t3932: F, t3948: F, t622: F, t6257: F, t626: F, t74: F, t79: F, t81: F, t8344: F, t8386: F, t8408: F) -> (F, F) {
    let t29528 = t3104 * t3085;
    let t29585 = -8.0 * t10495 * t8344 - 4.0 * t8408 * t3926 - 8.0 * t3113 * t10434 - 4.0 * t626 * t29466 - t74 * t29466 * t81 + t622 * t29466 * t81 / 2.0 - 4.0 * t1937 * t29484 * t81 + t1945 * t29484 * t81 / 2.0 - 2.0 * t79 * t29484 * t81 - 75.0 / 2.0 * t3948 * t8386 + 15.0 / 2.0 * t1924 * t3926 * t8386 + 15.0 / 2.0 * t3932 * t6257 + 85.0 / 4.0 * t10448 * t8386 - 4.0 * t3103 * t25201 - 5.0 / 2.0 * t10478 * t6257 - 19.0 / 8.0 * t21600 * t3931 * t8386;
    (t29528, t29585)
}
