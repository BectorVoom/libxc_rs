//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1469;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1470;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1471;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1472;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta355(t9924: f64, t9933: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t13125: f64, t13129: f64, t13132: f64, t13135: f64, t9853: f64, t9859: f64, t9907: f64, t9921: f64, t13093: f64, t13099: f64, t13111: f64, t225: f64, t68: f64, t822: f64, t1484: f64, t1891: f64, t2379: f64, t4119: f64, t845: f64, t776: f64, t2553: f64, t4226: f64, t12971: f64, t824: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t825: f64, t232: f64, t819: f64, t820: f64, t4162: f64, t816: f64, t1512: f64, t9671: f64, t9607: f64, t2697: f64, t4257: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13136, t13137, t13138) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1469(t9924, t9933, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t9853, t9859, t9907, t9921);
        let (t13141, t13151, t13157, t13160) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1470(t13093, t13099, t13111, t13138, t225, t68, t822, t1484, t1891, t2379, t4119, t845);
        let t13170 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1471(t13160, t776, t2553, t4226, t12971, t824, t13141, t13151, t13157, t1504, t1506, t228, t230, t2667, t2672, t2675, t4219, t4225, t4227, t4230, t822, t825);
        let (t13171, t13173, t13176) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1472(t13170, t232, t819, t820, t4162, t68);
        let (t13177, t13182, t13184, t13186, t13190, t13191) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1473(t13176, t816, t1512, t9671, t1484, t2379, t820, t9607, t2697, t4257, t4119, t776);
    (t13136, t13137, t13170, t13171, t13173, t13176, t13177, t13182, t13184, t13186, t13190, t13191)
}
