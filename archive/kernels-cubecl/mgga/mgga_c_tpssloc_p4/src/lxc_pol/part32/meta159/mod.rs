//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk838;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk839;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk840;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk841;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk842;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta159<F: Float>(t1530: F, t2752: F, t870: F, t193: F, t200: F, t1484: F, t262: F, t1877: F, t202: F, t2373: F, t2377: F, t2522: F, t4097: F, t4099: F, t4100: F, t4103: F, t4119: F, t4198: F, t4201: F, t4204: F, t4207: F, t4303: F, t766: F, t776: F, t868: F, t2523: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2538: F, t2665: F, t4209: F, t4213: F, t4214: F, t4215: F, t4216: F, t2: F, t265: F, t584: F, t1540: F, t690: F, t1409: F, t2770: F, t607: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4307, t4310, t4314) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk838::<F>(t1530, t2752, t870, t193, t200);
        let (t4315, t4319) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk839::<F>(t1484, t262, t1877, t193, t202, t2373, t2377, t2522, t4097, t4099, t4100, t4103, t4119, t4198, t4201, t4204, t4207, t4303, t4307, t4310, t4314, t766, t776, t868, t870);
        let t4323 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk840::<F>(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk841::<F>(t4319, t4323);
        let (t4332, t4335) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk842::<F>(t2, t265, t584, t1540, t690);
        let (t4337, t4338) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk843::<F>(t1409, t2770, t607);
    (t4307, t4310, t4314, t4315, t4324, t4332, t4335, t4337, t4338)
}
