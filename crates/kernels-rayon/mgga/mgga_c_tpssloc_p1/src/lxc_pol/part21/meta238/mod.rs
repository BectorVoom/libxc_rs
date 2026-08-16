//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta238(t3151: f64, t5392: f64, t974: f64, t5398: f64, t998: f64, t3146: f64, t1044: f64, t248: f64, t5681: f64, t225: f64, t5848: f64, t68: f64, t369: f64, t1539: f64, t1616: f64, t3071: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5884, t5885, t5889, t5890, t5893, t5894, t5900) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1413(t3151, t5392, t974, t5398, t998, t3146, t1044, t248, t5681);
        let (t5903, t5904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1414(t225, t5848, t68);
        let (t5905, t5908, t5909) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1415(t369, t5904, t1539, t1616, t3071);
    (t5884, t5885, t5889, t5890, t5893, t5894, t5900, t5903, t5904, t5905, t5908, t5909)
}
