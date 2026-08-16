//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1034;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1035;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta303(t21251: f64, t21255: f64, t21263: f64, t21265: f64, t21267: f64, t21270: f64, t21302: f64, t21305: f64, t21317: f64, t21320: f64, t21336: f64, t21372: f64, t21592: f64, t360: f64, t1021: f64, t248: f64, t1044: f64, t21134: f64, t21138: f64, t1020: f64, t1041: f64, t1622: f64, t17607: f64, t18042: f64, t21562: f64, t21566: f64, t21570: f64, t21574: f64, t21580: f64, t3070: f64, t4641: f64, t4644: f64, t5857: f64, t5861: f64, t5869: f64, t5900: f64, t973: f64, t21498: f64, t21529: f64, t21560: f64, t383: f64, t1625: f64, t5866: f64, t1060: f64, t1615: f64, t1932: f64, t5936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21593 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1034(t21251, t21255, t21263, t21265, t21267, t21270, t21302, t21305, t21317, t21320, t21336, t21372);
        let (t21594, t21595, t21597, t21603, t21609, t21612) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1035(t21592, t21593, t360, t1021, t248, t1044, t21134, t21138, t1020, t1041, t1622, t17607, t18042, t21562, t21566, t21570, t21574, t21580, t3070, t4641, t4644, t5857, t5861, t5869, t5900, t973);
        let (t21614, t21615, t21617, t21618, t21622, t21623) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1036(t21498, t21529, t21560, t21612, t383, t1625, t5866, t1060, t1615, t1932, t360, t5936);
    (t21594, t21595, t21597, t21603, t21609, t21614, t21615, t21617, t21618, t21622, t21623)
}
