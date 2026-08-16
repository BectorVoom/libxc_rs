//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta595(t11923: f64, t225: f64, t10913: f64, t11583: f64, t11570: f64, t1174: f64, t3471: f64, t698: f64, t3477: f64, t11504: f64, t135: f64, t43776: f64, t1186: f64, t2402: f64, t11498: f64, t457: f64, t625: f64, t221: f64, t456: f64, t461: f64, t11517: f64, t11539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44412, t44415, t44419, t44424, t44439, t44445, t44466) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174(t11923, t225, t10913, t11583, t11570, t1174, t3471, t698, t3477, t11504, t135, t43776);
        let (t44478, t44481, t44483, t44487, t44499) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2175(t1174, t1186, t2402, t11498, t135, t457, t625, t221, t456, t461, t11517, t11539);
    (t44412, t44415, t44419, t44424, t44439, t44445, t44466, t44478, t44481, t44483, t44487, t44499)
}
