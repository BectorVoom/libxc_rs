//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2362;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2363;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta605<F: Float>(t3684: F, t39354: F, t181: F, t2558: F, t686: F, t1291: F, t2369: F, t9720: F, t9843: F, t1294: F, t3814: F, t9874: F, t2411: F, t2414: F, t39246: F, t3691: F, t9494: F, t2508: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t39356, t39358, t39360, t39362, t39364, t39365) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2362::<F>(t3684, t39354, t181, t2558, t686, t1291, t2369, t9720, t9843, t1294, t3814, t9874);
        let t39373 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2363::<F>(t2411, t2414, t39246);
        let (t39374, t39377, t39378) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2364::<F>(t3691, t9494, t2508, t2369);
    (t39356, t39358, t39360, t39362, t39364, t39365, t39373, t39374, t39377, t39378)
}
