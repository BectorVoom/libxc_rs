//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1215;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta341<F: Float>(t2678: F, t828: F, t786: F, t9569: F, t805: F, t2610: F, t9541: F, t10041: F, t2563: F, t776: F, t222: F, t39934: F, t2617: F, t9637: F, t2649: F, t2691: F, t812: F, t815: F, t10003: F, t119: F, t13222: F, t13254: F, t210: F, t2633: F, t2643: F, t2647: F, t40848: F, t4178: F, t4180: F, t4182: F, t787: F, t9621: F, t9629: F, t9642: F, t9646: F, t9647: F, t2553: F, t2632: F, t10024: F, t809: F, t10017: F, t838: F, t2614: F, t2693: F, t238: F, t244: F, t248: F, t40445: F, t212: F, t2586: F, t9523: F, t9525: F, t9577: F, t116: F, t2379: F, t9529: F, t207: F, t40419: F, t9538: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41078, t41083, t41084, t41086, t41088, t41090, t41096) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1215::<F>(t2678, t828, t786, t9569, t805, t2610, t9541, t10041, t2563, t776, t222, t39934);
        let t41120 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216::<F>(t2617, t9637, t2649, t2691, t812, t815, t10003, t119, t13222, t13254, t210, t2633, t2643, t2647, t40848, t41078, t41084, t41086, t41088, t41090, t41096, t4178, t4180, t4182, t787, t9621, t9629, t9642, t9646, t9647);
        let (t41123, t41130, t41132, t41134, t41139) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217::<F>(t2553, t2632, t10024, t809, t10017, t838, t2614, t2693, t238, t244, t248, t40445);
        let (t41142, t41144, t41149, t41151, t41155) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1218::<F>(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t2563, t9529, t207, t40419, t9538);
    (t41083, t41120, t41123, t41130, t41132, t41134, t41139, t41142, t41144, t41149, t41151, t41155)
}
