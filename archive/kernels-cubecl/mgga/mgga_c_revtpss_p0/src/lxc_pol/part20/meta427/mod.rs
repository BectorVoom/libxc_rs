//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta427<F: Float>(t3599: F, t44169: F, t11239: F, t1204: F, t13041: F, t10326: F, t1042: F, t1214: F, t1261: F, t1264: F, t12705: F, t12855: F, t12933: F, t12938: F, t12945: F, t12956: F, t13048: F, t17202: F, t17235: F, t17454: F, t2258: F, t247: F, t3363: F, t3584: F, t3606: F, t3617: F, t3647: F, t3711: F, t3720: F, t43180: F, t43194: F, t43875: F, t5268: F, t5296: F, t5302: F, t3362: F, t3603: F, t2251: F, t12773: F, t12784: F, t13061: F, t10356: F, t12772: F, t12835: F, t3625: F, t13100: F, t828: F, t12699: F, t3624: F, t12257: F, t12277: F, t12726: F, t12777: F, t12787: F, t12809: F, t12810: F, t12910: F, t13065: F, t16696: F, t17459: F, t17550: F, t17688: F, t3626: F, t3631: F, t43793: F, t5331: F, t5340: F, t5405: F, t12841: F, t12879: F, t3630: F, t1260: F, t12975: F, t1247: F, t1251: F, t42994: F) -> (F, F, F, F, F, F, F, F) {
        let (t44173, t44185) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1603::<F>(t3599, t44169, t11239, t1204, t13041, t10326, t1042, t1214, t1261, t1264, t12705, t12855, t12933, t12938, t12945, t12956, t13048, t17202, t17235, t17454, t2258, t247, t3363, t3584, t3606, t3617, t3647, t3711, t3720, t43180, t43194, t43875, t5268, t5296, t5302);
        let (t44191, t44200, t44202, t44205, t44215) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604::<F>(t3362, t3603, t2251, t12773, t12784, t13061, t44173, t10356, t1214, t12772, t12835, t3625);
        let t44239 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1605::<F>(t13100, t828, t12699, t3624, t1042, t12257, t12277, t1261, t1264, t12705, t12726, t12777, t12784, t12787, t12809, t12810, t12910, t13065, t16696, t17459, t17550, t17688, t247, t3625, t3626, t3631, t3711, t3720, t43180, t43793, t44191, t44200, t44202, t44205, t44215, t5331, t5340, t5405);
        let (t44248, t44252, t44260, t44264) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1606::<F>(t12772, t12841, t5340, t12879, t828, t3625, t3630, t1260, t12975, t1247, t1251, t42994);
    (t44173, t44185, t44205, t44239, t44248, t44252, t44260, t44264)
}
