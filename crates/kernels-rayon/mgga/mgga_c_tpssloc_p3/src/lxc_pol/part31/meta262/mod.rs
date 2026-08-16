//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1095;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1096;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1097;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1098;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta262(t6966: f64, t6974: f64, t1338: f64, t2085: f64, t1352: f64, t553: f64, t7191: f64, t1332: f64, t1336: f64, t2089: f64, t544: f64, t6971: f64, t6980: f64, t6984: f64, t1378: f64, t1375: f64, t1386: f64, t2092: f64, t3758: f64, t3882: f64, t568: f64, t6893: f64, t6904: f64, t6909: f64, t7174: f64, t7176: f64, t7179: f64, t7192: f64, t7194: f64, t7199: f64, t533: f64, t1390: f64, t2095: f64, t6999: f64, t113: f64, t1266: f64, t1393: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t2096: f64, t2314: f64, t4034: f64, t510: f64, t574: f64, t650: f64, t652: f64, t672: f64, t6876: f64, t7040: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7156: f64, t7166: f64, t7171: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7202, t7204, t7208) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1095(t6966, t6974, t1338, t2085);
        let (t7209, t7211, t7213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1096(t1352, t7208, t553, t7191, t1332, t1336, t2089, t544, t6971, t6980, t6984, t7202, t7204);
        let t7214 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1097(t1378, t7213);
        let t7216 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1098(t1375, t1386, t2092, t3758, t3882, t568, t6893, t6904, t6909, t7174, t7176, t7179, t7192, t7194, t7199, t7214);
        let (t7217, t7218, t7220, t7222) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1099(t533, t7216, t1390, t2095, t6999, t113, t1266, t1393, t1983, t2036, t2040, t2075, t2079, t2096, t2314, t4034, t510, t574, t650, t652, t672, t6876, t7040, t7042, t7050, t7057, t7061, t7156, t7166, t7171);
    (t7202, t7204, t7208, t7209, t7211, t7213, t7214, t7216, t7217, t7218, t7220, t7222)
}
