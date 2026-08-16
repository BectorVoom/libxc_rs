//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta245 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1440;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1441;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1442;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1443;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1444;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta245(t1156: f64, t6068: f64, t3383: f64, t3390: f64, t4721: f64, t4770: f64, t5973: f64, t5977: f64, t5981: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64, t3403: f64, t1129: f64, t1148: f64, t1683: f64, t1695: f64, t3332: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4797: f64, t4835: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6031: f64, t6037: f64, t6053: f64, t6056: f64, t6064: f64, t300: f64, t1703: f64, t4869: f64, t3375: f64, t1164: f64, t1147: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6069 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1440(t1156, t6068);
        let t6084 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1441(t3383, t3390, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
        let t6085 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1442(t1156, t6084);
        let t6088 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1443(t3403, t6068);
        let t6091 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1444(t1129, t1148, t1683, t1695, t3332, t3357, t3376, t3401, t436, t4797, t4835, t5985, t5987, t5991, t6023, t6026, t6031, t6037, t6053, t6056, t6064, t6069, t6085, t6088);
        let (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1445(t300, t6091, t6064, t1703, t4869, t1156, t3375, t6068, t1164, t1147, t6084, t3400);
    (t6069, t6084, t6085, t6088, t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105)
}
