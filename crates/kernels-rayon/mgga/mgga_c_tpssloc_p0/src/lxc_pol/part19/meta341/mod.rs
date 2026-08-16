//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1215;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta341(t2678: f64, t828: f64, t786: f64, t9569: f64, t805: f64, t2610: f64, t9541: f64, t10041: f64, t2563: f64, t776: f64, t222: f64, t39934: f64, t2617: f64, t9637: f64, t2649: f64, t2691: f64, t812: f64, t815: f64, t10003: f64, t119: f64, t13222: f64, t13254: f64, t210: f64, t2633: f64, t2643: f64, t2647: f64, t40848: f64, t4178: f64, t4180: f64, t4182: f64, t787: f64, t9621: f64, t9629: f64, t9642: f64, t9646: f64, t9647: f64, t2553: f64, t2632: f64, t10024: f64, t809: f64, t10017: f64, t838: f64, t2614: f64, t2693: f64, t238: f64, t244: f64, t248: f64, t40445: f64, t212: f64, t2586: f64, t9523: f64, t9525: f64, t9577: f64, t116: f64, t2379: f64, t9529: f64, t207: f64, t40419: f64, t9538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41078, t41083, t41084, t41086, t41088, t41090, t41096) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1215(t2678, t828, t786, t9569, t805, t2610, t9541, t10041, t2563, t776, t222, t39934);
        let t41120 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216(t2617, t9637, t2649, t2691, t812, t815, t10003, t119, t13222, t13254, t210, t2633, t2643, t2647, t40848, t41078, t41084, t41086, t41088, t41090, t41096, t4178, t4180, t4182, t787, t9621, t9629, t9642, t9646, t9647);
        let (t41123, t41130, t41132, t41134, t41139) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217(t2553, t2632, t10024, t809, t10017, t838, t2614, t2693, t238, t244, t248, t40445);
        let (t41142, t41144, t41149, t41151, t41155) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1218(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t2563, t9529, t207, t40419, t9538);
    (t41083, t41120, t41123, t41130, t41132, t41134, t41139, t41142, t41144, t41149, t41151, t41155)
}
