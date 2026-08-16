//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2525;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta698(t4542: f64, t698: f64, t973: f64, t10186: f64, t13788: f64, t13560: f64, t699: f64, t2403: f64, t4392: f64, t13646: f64, t1553: f64, t9709: f64, t13538: f64, t133: f64, t135: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48066, t48068, t48087, t48096, t48098, t48103) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2525(t4542, t698, t973, t10186, t13788, t13560, t699, t2403, t4392, t13646, t1553, t9709);
        let (t48116, t48140) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2526(t13538, t699, t133, t135, t241);
    (t48066, t48068, t48087, t48096, t48098, t48103, t48116, t48140)
}
