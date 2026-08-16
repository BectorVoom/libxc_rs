//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1931;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1932;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta499<F: Float>(t21251: F, t21255: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21317: F, t21320: F, t21336: F, t21372: F, t21592: F, t360: F, t1021: F, t248: F, t1044: F, t21134: F, t21138: F, t1020: F, t1041: F, t1622: F, t17607: F, t18042: F, t21562: F, t21566: F, t21570: F, t21574: F, t21580: F, t3070: F, t4641: F, t4644: F, t5857: F, t5861: F, t5869: F, t5900: F, t973: F, t21498: F, t21529: F, t21560: F, t383: F, t1625: F, t5866: F, t1060: F, t1615: F, t1932: F, t5936: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t21593 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1931::<F>(t21251, t21255, t21263, t21265, t21267, t21270, t21302, t21305, t21317, t21320, t21336, t21372);
        let (t21594, t21595, t21597, t21603, t21609, t21612) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1932::<F>(t21592, t21593, t360, t1021, t248, t1044, t21134, t21138, t1020, t1041, t1622, t17607, t18042, t21562, t21566, t21570, t21574, t21580, t3070, t4641, t4644, t5857, t5861, t5869, t5900, t973);
        let (t21614, t21615, t21617, t21618, t21622, t21623) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1933::<F>(t21498, t21529, t21560, t21612, t383, t1625, t5866, t1060, t1615, t1932, t360, t5936);
    (t21594, t21595, t21597, t21603, t21609, t21614, t21615, t21617, t21618, t21622, t21623)
}
