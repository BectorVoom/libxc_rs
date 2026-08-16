//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1030;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta220(t381: f64, t5848: f64, t1603: f64, t1625: f64, t1044: f64, t248: f64, t5685: f64, t3062: f64, t5677: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64, t360: f64, t1021: f64, t1615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5849, t5851, t5857, t5861, t5866) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1030(t381, t5848, t1603, t1625, t1044, t248, t5685, t3062, t5677, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814);
        let (t5867, t5869, t5872) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1031(t360, t5866, t1021, t248, t1615);
    (t5849, t5851, t5857, t5861, t5866, t5867, t5869, t5872)
}
