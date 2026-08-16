//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1232;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1233;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1234;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1235;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1236;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1237;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta263(t466: f64, t7348: f64, t2145: f64, t225: f64, t1251: f64, t2154: f64, t3598: f64, t1170: f64, t2148: f64, t2121: f64, t7284: f64, t477: f64, t491: f64, t1090: f64, t1186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7349, t7351) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1232(t466, t7348, t2145, t225);
        let t7356 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1233(t1251, t2154, t3598);
        let (t7359, t7361, t7362) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1234(t1170, t2148, t2121, t225, t7284);
        let t7363 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1235(t477, t491);
        let t7364 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1236(t1090, t7363);
        let t7365 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1237(t7362, t7364);
        let t7368 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1238(t1186, t2148);
    (t7349, t7351, t7356, t7359, t7361, t7362, t7363, t7364, t7365, t7368)
}
