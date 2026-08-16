//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta264(t10277: f64, t344: f64, t9288: f64, t2979: f64, t338: f64, t9277: f64, t698: f64, t986: f64, t973: f64, t135: f64, t3010: f64, t241: f64, t625: f64, t281: f64, t283: f64, t2403: f64, t909: f64, t2827: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10279, t10280, t10283, t10286, t10287, t10289, t10290, t10292) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1413(t10277, t344, t9288, t2979, t338, t9277, t698, t986, t973, t135, t3010, t241, t625);
        let (t10294, t10295, t10296) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1414(t10292, t281, t283, t2403, t909);
        let t10298 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1415(t2827, t699);
    (t10279, t10280, t10283, t10286, t10287, t10289, t10290, t10292, t10294, t10295, t10296, t10298)
}
