//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1266;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1267;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1268;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1269;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1270;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta349(t10194: f64, t9258: f64, t123: f64, t2768: f64, t10277: f64, t39097: f64, t882: f64, t10537: f64, t690: f64, t2250: f64, t2771: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41690: f64, t41695: f64, t41699: f64, t41703: f64, t291: f64, t41677: f64, t10603: f64, t2929: f64, t4497: f64, t959: f64, t10713: f64, t2940: f64, t2904: f64, t952: f64, t2924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41705, t41707) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1266(t10194, t9258, t123, t2768);
        let (t41709, t41711) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1267(t10277, t39097, t123, t882);
        let t41713 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1268(t10537, t690);
        let (t41715, t41717) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1269(t2250, t2771, t123, t882);
        let t41719 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1270(t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let (t41722, t41726, t41728, t41732, t41733) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1271(t291, t41677, t41719, t10603, t2929, t4497, t959, t10713, t2940, t2904, t952, t2924);
    (t41705, t41707, t41709, t41711, t41713, t41715, t41717, t41722, t41726, t41728, t41732, t41733)
}
