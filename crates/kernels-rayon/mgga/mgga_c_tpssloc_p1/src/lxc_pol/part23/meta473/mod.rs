//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1415;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1416;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta473(t5398: f64, t5971: f64, t1088: f64, t123: f64, t1409: f64, t71176: f64, t3240: f64, t77973: f64, t63332: f64, t63334: f64, t63361: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t77989: f64, t77992: f64, t77995: f64, t43820: f64, t50834: f64, t71154: f64, t71156: f64, t77998: f64, t78002: f64, t78005: f64, t78033: f64, t78037: f64, t78041: f64, t1107: f64, t43880: f64, t78028: f64, t43777: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78043, t78045) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1413(t5398, t5971, t1088, t123);
        let (t78047, t78049) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1414(t1409, t71176, t1088, t123);
        let t78057 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1415(t123, t3240, t77973);
        let (t78064, t78076) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1416(t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057, t43820, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let (t78077, t78078, t78080, t78082) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417(t78064, t78076, t1107, t43880, t78028, t43777, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049);
    (t78043, t78045, t78047, t78049, t78057, t78077, t78078, t78080, t78082)
}
