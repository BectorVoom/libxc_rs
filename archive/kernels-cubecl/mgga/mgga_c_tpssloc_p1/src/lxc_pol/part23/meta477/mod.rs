//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1429;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta477<F: Float>(t22229: F, t4869: F, t6084: F, t1164: F, t3400: F, t3403: F, t21939: F, t4874: F, t1156: F, t3375: F, t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F, t44348: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t423: F, t21961: F, t51249: F, t11275: F, t3315: F, t78129: F, t6068: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t78242, t78243, t78247, t78250, t78254, t78266) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1429::<F>(t22229, t4869, t6084, t1164, t3400, t3403, t21939, t4874, t1156, t3375, t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057);
        let t78278 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430::<F>(t44348, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let (t78281, t78283, t78286, t78287) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431::<F>(t423, t78266, t78278, t21961, t51249, t11275, t3315, t78129, t6068);
    (t78242, t78243, t78247, t78250, t78254, t78281, t78283, t78286, t78287)
}
