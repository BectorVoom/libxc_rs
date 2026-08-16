//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1415;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1416;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta473<F: Float>(t5398: F, t5971: F, t1088: F, t123: F, t1409: F, t71176: F, t3240: F, t77973: F, t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t43820: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t1107: F, t43880: F, t78028: F, t43777: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t78043, t78045) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1413::<F>(t5398, t5971, t1088, t123);
        let (t78047, t78049) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1414::<F>(t1409, t71176, t1088, t123);
        let t78057 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1415::<F>(t123, t3240, t77973);
        let (t78064, t78076) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1416::<F>(t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057, t43820, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let (t78077, t78078, t78080, t78082) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417::<F>(t78064, t78076, t1107, t43880, t78028, t43777, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049);
    (t78043, t78045, t78047, t78049, t78057, t78077, t78078, t78080, t78082)
}
