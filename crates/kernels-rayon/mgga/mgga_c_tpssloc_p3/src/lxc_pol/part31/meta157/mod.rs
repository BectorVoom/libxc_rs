//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta157 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk780;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk781;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk782;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk783;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk784;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta157(t1530: f64, t2752: f64, t870: f64, t193: f64, t200: f64, t1484: f64, t262: f64, t1877: f64, t202: f64, t2373: f64, t2377: f64, t2522: f64, t4097: f64, t4099: f64, t4100: f64, t4103: f64, t4119: f64, t4198: f64, t4201: f64, t4204: f64, t4207: f64, t4303: f64, t766: f64, t776: f64, t868: f64, t2523: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64, t4209: f64, t4213: f64, t4214: f64, t4215: f64, t4216: f64, t2: f64, t265: f64, t584: f64, t1540: f64, t690: f64, t1409: f64, t2770: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4307, t4310, t4314) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk780(t1530, t2752, t870, t193, t200);
        let (t4315, t4319) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk781(t1484, t262, t1877, t193, t202, t2373, t2377, t2522, t4097, t4099, t4100, t4103, t4119, t4198, t4201, t4204, t4207, t4303, t4307, t4310, t4314, t766, t776, t868, t870);
        let t4323 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk782(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk783(t4319, t4323);
        let (t4332, t4335) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk784(t2, t265, t584, t1540, t690);
        let (t4337, t4338) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk785(t1409, t2770, t607);
    (t4307, t4310, t4314, t4315, t4324, t4332, t4335, t4337, t4338)
}
