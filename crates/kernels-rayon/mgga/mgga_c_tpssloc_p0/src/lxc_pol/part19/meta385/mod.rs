//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1442;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta385(t11292: f64, t11433: f64, t1164: f64, t3377: f64, t11285: f64, t43679: f64, t44154: f64, t11923: f64, t225: f64, t10913: f64, t11583: f64, t11570: f64, t1174: f64, t3471: f64, t698: f64, t3475: f64, t3469: f64, t3477: f64, t11504: f64, t135: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43754: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43835: f64, t43776: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t1186: f64, t2402: f64, t11498: f64, t457: f64, t625: f64, t221: f64, t456: f64, t461: f64, t11496: f64, t11569: f64, t11575: f64, t1184: f64, t15288: f64, t3447: f64, t3449: f64, t460: f64, t4934: f64, t974: f64, t11517: f64, t11539: f64, t11521: f64, t3431: f64, t15394: f64, t11147: f64, t9288: f64, t11588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44396, t44400, t44412, t44415, t44419) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1442(t11292, t11433, t1164, t3377, t11285, t43679, t44154, t11923, t225, t10913, t11583, t11570);
        let (t44424, t44426, t44432, t44439, t44445, t44457) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443(t1174, t3471, t698, t3475, t3469, t3477, t11504, t135, t43713, t43717, t43721, t43725, t43754, t43759, t43766, t43768, t43770, t43773, t43835);
        let t44470 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444(t43776, t43837, t43839, t43842, t43845, t43848, t43851, t43855, t43857, t43859, t43861, t43863);
        let (t44483, t44493) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445(t1174, t1186, t2402, t11498, t135, t457, t625, t221, t456, t461, t11496, t11569, t11575, t1184, t15288, t3447, t3449, t44415, t44419, t44424, t44426, t44432, t44439, t44445, t44457, t44470, t460, t4934, t974);
        let (t44499, t44502, t44504, t44506, t44510) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1446(t11517, t11539, t1174, t11521, t3431, t1184, t15394, t11147, t460, t9288, t11588, t3469);
    (t44396, t44400, t44412, t44483, t44493, t44499, t44502, t44504, t44506, t44510)
}
