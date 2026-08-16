//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta129 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk859;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk860;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk861;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk862;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk863;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk864;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta129(t1373: f64, t225: f64, t1376: f64, t566: f64, t68: f64, t3787: f64, t562: f64, t1338: f64, t1372: f64, t193: f64, t532: f64, t1388: f64, t1390: f64, t531: f64, t571: f64, t112: f64, t1395: f64, t111: f64, t576: f64, t2218: f64, t2221: f64, t2225: f64, t2232: f64, t1406: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3882 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk859(t1373, t225);
        let t3887 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk860(t1376, t566, t68);
        let (t3897, t3901) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk861(t3787, t562, t1338, t1372);
        let t3918 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk862(t193, t532);
        let t3919 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk863(t1388, t1390);
        let (t3924, t3938, t3941) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk864(t531, t571, t112, t1395, t111, t576);
        let (t3951, t3953) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk865(t2218, t2221, t2225, t2232, t1406, t604);
    (t3882, t3887, t3897, t3901, t3918, t3919, t3924, t3938, t3941, t3951, t3953)
}
