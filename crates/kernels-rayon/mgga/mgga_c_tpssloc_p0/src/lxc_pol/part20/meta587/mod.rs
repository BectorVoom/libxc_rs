//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2160;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2161;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2162;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2163;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2164;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta587(t11149: f64, t690: f64, t11545: f64, t241: f64, t3241: f64, t11229: f64, t699: f64, t11232: f64, t242: f64, t281: f64, t415: f64, t2394: f64, t3253: f64, t3249: f64, t11155: f64, t11164: f64, t11173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2160(t11149, t690);
        let (t43761, t43763, t43768, t43770, t43776, t43777, t43780) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2161(t11545, t241, t3241, t11229, t699, t11232, t242, t281, t415, t2394, t3253);
        let t43782 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2162(t2394, t3249);
        let t43784 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2163(t11155, t690);
        let t43786 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2164(t11164, t690);
        let t43788 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2165(t11173, t690);
    (t43750, t43761, t43763, t43768, t43770, t43776, t43777, t43780, t43782, t43784, t43786, t43788)
}
