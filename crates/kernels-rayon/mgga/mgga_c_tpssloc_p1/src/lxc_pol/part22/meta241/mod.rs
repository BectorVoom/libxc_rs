//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta241 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1330;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1331;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1332;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1333;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1334;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta241(t10213: f64, t974: f64, t2769: f64, t632: f64, t344: f64, t698: f64, t976: f64, t979: f64, t973: f64, t135: f64, t2978: f64, t4509: f64, t984: f64, t2770: f64, t343: f64, t2775: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10214, t10216) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1330(t10213, t974, t2769, t632);
        let (t10217, t10224) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1331(t10216, t344, t698, t976);
        let (t10226, t10231) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1332(t10224, t979, t973, t135, t2978);
        let (t10235, t10236) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1333(t4509, t984, t2770, t343);
        let t10254 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1334(t2775, t343);
        let (t10276, t10277) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1335(t2769, t40);
    (t10214, t10216, t10217, t10224, t10226, t10231, t10235, t10236, t10254, t10276, t10277)
}
