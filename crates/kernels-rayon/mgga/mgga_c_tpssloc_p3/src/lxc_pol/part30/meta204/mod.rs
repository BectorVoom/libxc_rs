//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta204 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk963;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk964;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk965;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk966;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk967;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk968;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta204(t1297: f64, t1390: f64, t193: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5167: f64, t5169: f64, t5187: f64, t5263: f64, t5265: f64, t5267: f64, t5268: f64, t5269: f64, t533: f64, t5356: f64, t5165: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t2314: f64, t4026: f64, t4028: f64, t4034: f64, t4037: f64, t4073: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t3: f64, t112: f64, t1851: f64, t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t577: f64, t2218: f64, t2220: f64, t2222: f64, t2224: f64, t2226: f64, t2228: f64, t2232: f64, t1437: f64, t1409: f64, t65: f64, t11: f64, t2219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5360 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk963(t1297, t1390, t193, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5167, t5169, t5187, t5263, t5265, t5267, t5268, t5269, t533, t5356);
        let (t5361, t5363) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk964(t5165, t5360, t113, t1266, t1271, t1393, t1442, t1459, t1774, t1778, t1849, t2314, t4026, t4028, t4034, t4037, t4073, t4077, t510, t5107, t5118, t513, t574, t650, t652, t672);
        let (t5364, t5371) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk965(t3, t5363, t112, t1851);
        let (t5376, t5381, t5385) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk966(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2218, t2220, t2222, t2224, t2226, t2228, t2232);
        let t5389 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk967(t1437);
        let t5392 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk968(t1409);
        let (t5393, t5396, t5397) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk969(t5392, t65, t11, t2219);
    (t5361, t5363, t5364, t5371, t5376, t5381, t5385, t5389, t5392, t5393, t5396, t5397)
}
