//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk762;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta152(t2229: f64, t61: f64, t119: f64, t212: f64, t252: f64, t828: f64, t1929: f64, t343: f64, t984: f64, t3034: f64, t334: f64, rho0: f64, t371: f64, t533: f64, t556: f64, t1351: f64, t562: f64, t1388: f64, t3701: f64, t1184: f64, t460: f64, t590: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6597, t6600, t6647, t6720, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk762(t2229, t61, t119, t212, t252, t828, t1929, t343, t984, t3034, t334, rho0);
        let (t6793, t6924, t6977, t6999, t7319, t8705) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk763(t334, t371, t533, t556, t1351, t562, t1388, t3701, t1184, t460, t590, t60);
    (t6597, t6600, t6647, t6720, t6733, t6739, t6793, t6924, t6977, t6999, t7319, t8705)
}
