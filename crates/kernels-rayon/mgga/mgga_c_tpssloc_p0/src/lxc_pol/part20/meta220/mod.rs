//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1289;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta220(t334: f64, t371: f64, t533: f64, t556: f64, t1351: f64, t562: f64, t1388: f64, t3701: f64, t1184: f64, t460: f64, t1433: f64, t71: f64, t590: f64, t60: f64, t93: f64, t101: f64, t584: f64, t16: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6793, t6924, t6977, t6999, t7319, t7445) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1289(t334, t371, t533, t556, t1351, t562, t1388, t3701, t1184, t460, t1433, t71);
        let (t8705, t9108, t9174, t9211, t9212) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1290(t590, t60, t93, t101, t584, t16, t2);
    (t6793, t6924, t6977, t6999, t7319, t7445, t8705, t9108, t9174, t9211, t9212)
}
