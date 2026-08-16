//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta264 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1239;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1240;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1241;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1242;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1243;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta264(t50: f64, t6794: f64, t131: f64, t467: f64, t1009: f64, t461: f64, t1209: f64, t475: f64, t68: f64, t1245: f64, t1235: f64, t2147: f64, t462: f64, t1215: f64, t2144: f64, t1246: f64, t493: f64, t7348: f64, t1201: f64, t1244: f64, t2121: f64, t2152: f64, t470: f64, t7283: f64, t7361: f64, t7365: f64, t7368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7371, t7372, t7373) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1239(t50, t6794, t131, t467);
        let t7375 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1240(t1009, t461, t1209);
        let t7376 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1241(t475, t68);
        let (t7377, t7378) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1242(t1245, t7376, t7375);
        let t7381 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1243(t1235, t2147);
        let (t7382, t7387, t7389, t7391) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1244(t462, t7381, t1215, t2144, t1246, t493, t7348, t1201, t1244, t2121, t2152, t470, t7283, t7361, t7365, t7368, t7373, t7378);
    (t7371, t7372, t7373, t7375, t7376, t7377, t7378, t7381, t7382, t7387, t7389, t7391)
}
