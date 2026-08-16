//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta64 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk432;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk433;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk434;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk435;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk436;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk437;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk438;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta64<F: Float>(t1090: F, t1230: F, t248: F, t1174: F, t1195: F, t1198: F, t1203: F, t1213: F, t1218: F, t1224: F, t1227: F, t488: F, t466: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F, t1206: F, t1215: F, t491: F, t357: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1232 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk432::<F>(t1090, t1230, t248);
        let t1235 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk433::<F>(t1174, t1195, t1198, t1203, t1213, t1218, t1224, t1227, t1232, t488);
        let (t1236, t1238) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk434::<F>(t1235, t466, t225, t492);
        let (t1239, t1240) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk435::<F>(t496);
        let t1241 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk436::<F>(t1240, t68);
        let t1243 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk437::<F>(t1011, t1209);
        let t1244 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk438::<F>(t1206, t1243);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk439::<F>(t1215, t491, t357, t475);
    (t1232, t1235, t1236, t1238, t1239, t1240, t1241, t1243, t1244, t1245, t1246)
}
