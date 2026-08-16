//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta207 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1199;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1200;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1201;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1202;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1203;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta207<F: Float>(t1156: F, t6068: F, t3383: F, t3390: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F, t3403: F, t1129: F, t1148: F, t1683: F, t1695: F, t3332: F, t3357: F, t3376: F, t3401: F, t436: F, t4797: F, t4835: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6031: F, t6037: F, t6053: F, t6056: F, t6064: F, t300: F, t1703: F, t4869: F, t3375: F, t1164: F, t1147: F, t3400: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6069 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1199::<F>(t1156, t6068);
        let t6084 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1200::<F>(t3383, t3390, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
        let t6085 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1201::<F>(t1156, t6084);
        let t6088 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1202::<F>(t3403, t6068);
        let t6091 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1203::<F>(t1129, t1148, t1683, t1695, t3332, t3357, t3376, t3401, t436, t4797, t4835, t5985, t5987, t5991, t6023, t6026, t6031, t6037, t6053, t6056, t6064, t6069, t6085, t6088);
        let (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1204::<F>(t300, t6091, t6064, t1703, t4869, t1156, t3375, t6068, t1164, t1147, t6084, t3400);
    (t6069, t6084, t6085, t6088, t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105)
}
