//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk310;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk311;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk312;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk313;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta48(t1147: f64, t440: f64, t1086: f64, t1111: f64, t448: f64, t300: f64, t134: f64, t457: f64, t461: f64, t221: f64, t456: f64, t51: f64, t972: f64, t404: f64, t405: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1148, t1150, t1153, t1156) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk310(t1147, t440, t1086, t1111, t448);
        let t1164 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk311(t300, t440);
        let (t1169, t1171, t1173, t1174) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk312(t134, t457, t461, t221, t456, t51, t972);
        let t1176 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk313(t404, t405);
        let t1177 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk314(t1176, t974);
    (t1148, t1150, t1153, t1156, t1164, t1169, t1171, t1173, t1174, t1176, t1177)
}
