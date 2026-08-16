//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta268 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1217;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1218;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1219;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1220;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1221;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1222;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1223;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta268(t1245: f64, t7376: f64, t7375: f64, t1235: f64, t2147: f64, t462: f64, t1215: f64, t2144: f64, t1246: f64, t493: f64, t7348: f64, t1201: f64, t1244: f64, t2121: f64, t2152: f64, t470: f64, t7283: f64, t7361: f64, t7365: f64, t7368: f64, t7373: f64, t1241: f64, t1238: f64, t1252: f64, t2155: f64, t3487: f64, t3593: f64, t498: f64, t7282: f64, t7288: f64, t7291: f64, t7296: f64, t7303: f64, t7306: f64, t7349: f64, t7351: f64, t7356: f64, t2157: f64, t3640: f64, t28: f64, t265: f64, t504: f64, t1254: f64, t1256: f64, t193: f64, t336: f64, t4700: f64, t6834: f64, t2161: f64, t52: f64, t607: f64, t6855: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t7279: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7377, t7378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1217(t1245, t7376, t7375);
        let t7381 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1218(t1235, t2147);
        let (t7382, t7387, t7389, t7391) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1219(t462, t7381, t1215, t2144, t1246, t493, t7348, t1201, t1244, t2121, t2152, t470, t7283, t7361, t7365, t7368, t7373, t7378);
        let t7392 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1220(t1241, t7391);
        let t7394 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1221(t1238, t1252, t2121, t2155, t3487, t3593, t498, t7282, t7283, t7288, t7291, t7296, t7303, t7306, t7349, t7351, t7356, t7392);
        let t7398 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1222(t2157, t3640);
        let (t7402, t7407) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1223(t28, t265, t504, t1254, t1256, t193, t336, t4700, t6834, t7394, t7398, t2161, t52, t607, t6855, dens_threshold, rho1, zeta_threshold);
        let t7408 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1224(t7279, t7407);
    (t7377, t7378, t7381, t7382, t7387, t7389, t7391, t7392, t7394, t7398, t7402, t7408)
}
