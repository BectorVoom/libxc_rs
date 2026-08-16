//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1523;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1524;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1525;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1526;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta417(t19804: f64, t562: f64, t1372: f64, t6361: f64, t225: f64, t6435: f64, t1323: f64, t6434: f64, t1385: f64, t6439: f64, t12021: f64, t6362: f64, t1375: f64, t1386: f64, t16022: f64, t16460: f64, t1843: f64, t3758: f64, t3882: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64, t6440: f64, t6461: f64, t20034: f64, t1390: f64, t6463: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t1297: f64, t1307: f64, t1388: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t193: f64, t19596: f64, t19599: f64, t19603: f64, t19631: f64, t19677: f64, t3918: f64, t5126: f64, t5160: f64, t5161: f64, t533: f64, t5356: f64, t571: f64, t6330: f64, t9780: f64, t9789: f64, t3701: f64, t6324: f64, t6347: f64, t12461: f64, t12087: f64, t12094: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t16497: f64, t1799: f64, t19678: f64, t19683: f64, t19684: f64, t19685: f64, t19686: f64, t19687: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t3919: f64, t12116: f64, t12118: f64, t12123: f64, t12130: f64, t12133: f64, t12141: f64, t15976: f64, t16171: f64, t19689: f64, t19690: f64, t19691: f64, t19693: f64, t19694: f64, t19695: f64, t19696: f64, t19697: f64, t19698: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20038, t20040, t20044, t20048, t20050, t20051, t20060) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1523(t19804, t562, t1372, t6361, t225, t6435, t1323, t6434, t1385, t6439, t12021, t6362);
        let t20062 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1524(t1375, t1386, t16022, t16460, t1843, t20038, t20040, t20044, t20048, t20051, t20060, t3758, t3882, t5215, t5326, t5354, t568, t6440, t6461);
        let (t20063, t20075) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1525(t20034, t20062, t1390, t6463, t12044, t12048, t12057, t12059, t1297, t1307, t1388, t15898, t15911, t15916, t15917, t15923, t193, t19596, t19599, t19603, t19631, t19677, t3918, t5126, t5160, t5161, t533, t5356, t571, t6330, t9780, t9789);
        let (t20085, t20092) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1526(t3701, t6324, t571, t6347, t12461, t12087, t12094, t12103, t12105, t12109, t12114, t1307, t1388, t16497, t1799, t19678, t19683, t19684, t19685, t19686, t19687, t3918, t5126, t5160, t9793, t9797, t9820, t9824);
        let t20096 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1527(t3919, t6330, t12116, t12118, t12123, t12130, t12133, t12141, t15976, t16171, t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t5126, t9853, t9859);
    (t20038, t20040, t20044, t20048, t20050, t20051, t20060, t20063, t20075, t20085, t20092, t20096)
}
