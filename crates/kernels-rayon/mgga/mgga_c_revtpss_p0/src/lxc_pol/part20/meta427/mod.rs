//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta427(t3599: f64, t44169: f64, t11239: f64, t1204: f64, t13041: f64, t10326: f64, t1042: f64, t1214: f64, t1261: f64, t1264: f64, t12705: f64, t12855: f64, t12933: f64, t12938: f64, t12945: f64, t12956: f64, t13048: f64, t17202: f64, t17235: f64, t17454: f64, t2258: f64, t247: f64, t3363: f64, t3584: f64, t3606: f64, t3617: f64, t3647: f64, t3711: f64, t3720: f64, t43180: f64, t43194: f64, t43875: f64, t5268: f64, t5296: f64, t5302: f64, t3362: f64, t3603: f64, t2251: f64, t12773: f64, t12784: f64, t13061: f64, t10356: f64, t12772: f64, t12835: f64, t3625: f64, t13100: f64, t828: f64, t12699: f64, t3624: f64, t12257: f64, t12277: f64, t12726: f64, t12777: f64, t12787: f64, t12809: f64, t12810: f64, t12910: f64, t13065: f64, t16696: f64, t17459: f64, t17550: f64, t17688: f64, t3626: f64, t3631: f64, t43793: f64, t5331: f64, t5340: f64, t5405: f64, t12841: f64, t12879: f64, t3630: f64, t1260: f64, t12975: f64, t1247: f64, t1251: f64, t42994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44173, t44185) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603(t3599, t44169, t11239, t1204, t13041, t10326, t1042, t1214, t1261, t1264, t12705, t12855, t12933, t12938, t12945, t12956, t13048, t17202, t17235, t17454, t2258, t247, t3363, t3584, t3606, t3617, t3647, t3711, t3720, t43180, t43194, t43875, t5268, t5296, t5302);
        let (t44191, t44200, t44202, t44205, t44215) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604(t3362, t3603, t2251, t12773, t12784, t13061, t44173, t10356, t1214, t12772, t12835, t3625);
        let t44239 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605(t13100, t828, t12699, t3624, t1042, t12257, t12277, t1261, t1264, t12705, t12726, t12777, t12784, t12787, t12809, t12810, t12910, t13065, t16696, t17459, t17550, t17688, t247, t3625, t3626, t3631, t3711, t3720, t43180, t43793, t44191, t44200, t44202, t44205, t44215, t5331, t5340, t5405);
        let (t44248, t44252, t44260, t44264) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1606(t12772, t12841, t5340, t12879, t828, t3625, t3630, t1260, t12975, t1247, t1251, t42994);
    (t44173, t44185, t44205, t44239, t44248, t44252, t44260, t44264)
}
