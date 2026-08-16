//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1442;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta385<F: Float>(t11292: F, t11433: F, t1164: F, t3377: F, t11285: F, t43679: F, t44154: F, t11923: F, t225: F, t10913: F, t11583: F, t11570: F, t1174: F, t3471: F, t698: F, t3475: F, t3469: F, t3477: F, t11504: F, t135: F, t43713: F, t43717: F, t43721: F, t43725: F, t43754: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43835: F, t43776: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t1186: F, t2402: F, t11498: F, t457: F, t625: F, t221: F, t456: F, t461: F, t11496: F, t11569: F, t11575: F, t1184: F, t15288: F, t3447: F, t3449: F, t460: F, t4934: F, t974: F, t11517: F, t11539: F, t11521: F, t3431: F, t15394: F, t11147: F, t9288: F, t11588: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44396, t44400, t44412, t44415, t44419) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1442::<F>(t11292, t11433, t1164, t3377, t11285, t43679, t44154, t11923, t225, t10913, t11583, t11570);
        let (t44424, t44426, t44432, t44439, t44445, t44457) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443::<F>(t1174, t3471, t698, t3475, t3469, t3477, t11504, t135, t43713, t43717, t43721, t43725, t43754, t43759, t43766, t43768, t43770, t43773, t43835);
        let t44470 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444::<F>(t43776, t43837, t43839, t43842, t43845, t43848, t43851, t43855, t43857, t43859, t43861, t43863);
        let (t44483, t44493) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445::<F>(t1174, t1186, t2402, t11498, t135, t457, t625, t221, t456, t461, t11496, t11569, t11575, t1184, t15288, t3447, t3449, t44415, t44419, t44424, t44426, t44432, t44439, t44445, t44457, t44470, t460, t4934, t974);
        let (t44499, t44502, t44504, t44506, t44510) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1446::<F>(t11517, t11539, t1174, t11521, t3431, t1184, t15394, t11147, t460, t9288, t11588, t3469);
    (t44396, t44400, t44412, t44483, t44493, t44499, t44502, t44504, t44506, t44510)
}
