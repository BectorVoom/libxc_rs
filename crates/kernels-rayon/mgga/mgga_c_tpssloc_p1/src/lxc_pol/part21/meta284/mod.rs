//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1576;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta284(t698: f64, t986: f64, t973: f64, t135: f64, t3010: f64, t241: f64, t625: f64, t281: f64, t283: f64, t2403: f64, t909: f64, t2827: f64, t699: f64, t2830: f64, t2833: f64, t2978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10286, t10287, t10290, t10292, t10294, t10295, t10296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1576(t698, t986, t973, t135, t3010, t241, t625, t281, t283, t2403, t909);
        let (t10298, t10300, t10302, t10304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1577(t2827, t699, t2830, t2833, t241, t2978);
    (t10286, t10287, t10290, t10292, t10294, t10295, t10296, t10298, t10300, t10302, t10304)
}
