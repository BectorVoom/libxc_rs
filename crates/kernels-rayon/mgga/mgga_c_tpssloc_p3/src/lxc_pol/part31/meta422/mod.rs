//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1540;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1541;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta422(t22674: f64, t6907: f64, t6897: f64, t131: f64, t557: f64, t209: f64, t1878: f64, t212: f64, t225: f64, t6968: f64, t22642: f64, t268: f64, t534: f64, t6559: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22675, t22676, t22683, t22684, t22685, t22690) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1540(t22674, t6907, t6897, t131, t557, t209, t1878, t212, t225);
        let (t22691, t22692, t22704) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1541(t22690, t6968, t22642, t268, t534, t6559);
        let t22705 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1542(t1338, t22690);
    (t22675, t22676, t22683, t22684, t22685, t22690, t22691, t22692, t22704, t22705)
}
