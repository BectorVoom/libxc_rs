//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta595<F: Float>(t11923: F, t225: F, t10913: F, t11583: F, t11570: F, t1174: F, t3471: F, t698: F, t3477: F, t11504: F, t135: F, t43776: F, t1186: F, t2402: F, t11498: F, t457: F, t625: F, t221: F, t456: F, t461: F, t11517: F, t11539: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44412, t44415, t44419, t44424, t44439, t44445, t44466) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174::<F>(t11923, t225, t10913, t11583, t11570, t1174, t3471, t698, t3477, t11504, t135, t43776);
        let (t44478, t44481, t44483, t44487, t44499) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2175::<F>(t1174, t1186, t2402, t11498, t135, t457, t625, t221, t456, t461, t11517, t11539);
    (t44412, t44415, t44419, t44424, t44439, t44445, t44466, t44478, t44481, t44483, t44487, t44499)
}
