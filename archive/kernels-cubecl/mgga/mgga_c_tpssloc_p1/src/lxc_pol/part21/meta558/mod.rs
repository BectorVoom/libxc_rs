//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta558<F: Float>(t1164: F, t18926: F, t4869: F, t4875: F, t18711: F, t300: F, t3375: F, t6084: F, t1157: F, t3411: F, t6102: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18837: F, t18839: F, t18917: F, t18920: F, t18922: F, t18924: F) -> (F, F, F, F, F, F, F, F) {
        let (t18928, t18930, t18932, t18933, t18934, t18936, t18938, t18939) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2262::<F>(t1164, t18926, t4869, t4875, t18711, t300, t3375, t6084, t1157, t3411, t6102, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924);
    (t18928, t18930, t18932, t18933, t18934, t18936, t18938, t18939)
}
