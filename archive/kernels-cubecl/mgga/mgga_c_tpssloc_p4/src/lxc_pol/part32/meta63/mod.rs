//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta63 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk412;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk413;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk414;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk415;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk416;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk417;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta63<F: Float>(t1196: F, t607: F, t974: F, t1190: F, t225: F, t68: F, t484: F, t1009: F, t466: F, t1011: F, t476: F, t478: F, t1017: F, t483: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1197, t1198) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk412::<F>(t1196, t607, t974);
        let t1201 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk413::<F>(t1190, t225);
        let t1202 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk414::<F>(t1201, t68);
        let (t1203, t1206, t1207) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk415::<F>(t1202, t484, t1009, t466, t1011);
        let (t1208, t1209) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk416::<F>(t476);
        let t1210 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk417::<F>(t1209, t478);
        let (t1211, t1212) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk418::<F>(t1017, t483, t1210);
    (t1197, t1198, t1201, t1202, t1203, t1206, t1207, t1208, t1209, t1210, t1211, t1212)
}
