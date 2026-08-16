//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1429;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta477(t22229: f64, t4869: f64, t6084: f64, t1164: f64, t3400: f64, t3403: f64, t21939: f64, t4874: f64, t1156: f64, t3375: f64, t63332: f64, t63334: f64, t63361: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t77989: f64, t77992: f64, t77995: f64, t78057: f64, t44348: f64, t50834: f64, t71154: f64, t71156: f64, t77998: f64, t78002: f64, t78005: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64, t423: f64, t21961: f64, t51249: f64, t11275: f64, t3315: f64, t78129: f64, t6068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78242, t78243, t78247, t78250, t78254, t78266) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1429(t22229, t4869, t6084, t1164, t3400, t3403, t21939, t4874, t1156, t3375, t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057);
        let t78278 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430(t44348, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let (t78281, t78283, t78286, t78287) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431(t423, t78266, t78278, t21961, t51249, t11275, t3315, t78129, t6068);
    (t78242, t78243, t78247, t78250, t78254, t78281, t78283, t78286, t78287)
}
