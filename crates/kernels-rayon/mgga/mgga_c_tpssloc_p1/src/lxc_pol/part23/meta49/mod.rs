//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta49 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk315;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk316;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk317;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta49(t1089: f64, t461: f64, t1169: f64, t221: f64, t456: f64, t1176: f64, t1009: f64, t466: f64, t1011: f64, t476: f64, t478: f64, t1017: f64, t483: f64, t486: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1178, t1193, t1195, t1196, t1206, t1207, t1208, t1209) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk315(t1089, t461, t1169, t221, t456, t1176, t1009, t466, t1011, t476);
        let t1210 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk316(t1209, t478);
        let (t1212, t1213) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk317(t1017, t483, t1210, t1207);
        let t1214 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk318(t486, t61);
    (t1178, t1193, t1195, t1196, t1206, t1207, t1208, t1209, t1210, t1212, t1213, t1214)
}
