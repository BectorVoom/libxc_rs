//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2560;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta718(t10480: f64, t13969: f64, t13986: f64, t3039: f64, t4599: f64, t49850: f64, t10870: f64, t4644: f64, t10875: f64, t48569: f64, t10937: f64, t13765: f64, t10903: f64, t14507: f64, t14651: f64, t3069: f64, t10956: f64, t1611: f64, t10517: f64, t4630: f64, t10459: f64, t4608: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50255, t50258, t50262, t50265, t50272) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2560(t10480, t13969, t13986, t3039, t4599, t49850, t10870, t4644, t10875, t48569, t10937, t13765);
        let (t50302, t50324, t50334, t50337, t50343, t50361) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2561(t10903, t14507, t14651, t3069, t10956, t1611, t10517, t4630, t10459, t4644, t4608, t698, t973);
    (t50255, t50258, t50262, t50265, t50272, t50302, t50324, t50334, t50337, t50343, t50361)
}
