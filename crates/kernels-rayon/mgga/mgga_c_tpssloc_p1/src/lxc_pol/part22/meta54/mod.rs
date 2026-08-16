//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta54 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk383;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk384;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk385;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk386;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk387;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk388;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta54(t1086: f64, t154: f64, t486: f64, t636: f64, t607: f64, t123: f64, t423: f64, t419: f64, t409: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk383(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk384(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk385(t1089, t607);
        let (t1091, t1092, t1094) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk386(t1088, t1090, t123, t1087);
        let (t1096, t1097, t1098) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk387(t1094, t423, t419);
        let t1099 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk388(t1098, t409);
        let t1100 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk389(t410);
    (t1087, t1088, t1089, t1090, t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100)
}
