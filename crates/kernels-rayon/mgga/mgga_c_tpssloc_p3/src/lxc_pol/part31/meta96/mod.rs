//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk590;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk591;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk592;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk593;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk594;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk595;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk596;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta96(t2085: f64, t539: f64, t553: f64, t2011: f64, t544: f64, t1378: f64, t1375: f64, t1989: f64, t568: f64, t533: f64, t1390: f64, t113: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t510: f64, t574: f64, t652: f64, t3: f64, t1401: f64, t2039: f64, t577: f64, t11: f64, t2: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2086, t2089) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk590(t2085, t539, t553);
        let t2091 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk591(t2011, t2089, t544);
        let t2092 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk592(t1378, t2091);
        let t2094 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk593(t1375, t1989, t2086, t2092, t568);
        let t2095 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk594(t2094, t533);
        let t2096 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk595(t1390, t2095);
        let t2098 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk596(t113, t1983, t2036, t2040, t2075, t2079, t2096, t510, t574, t652);
        let (t2099, t2105, t2218, t2219) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk597(t2098, t3, t1401, t2039, t577, t11, t2, t584);
    (t2086, t2089, t2091, t2092, t2094, t2095, t2096, t2098, t2099, t2105, t2218, t2219)
}
