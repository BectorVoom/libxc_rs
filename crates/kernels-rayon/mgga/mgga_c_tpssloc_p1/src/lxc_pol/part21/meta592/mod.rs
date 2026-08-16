//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2337;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2338;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta592(t19804: f64, t562: f64, t1372: f64, t6361: f64, t225: f64, t6435: f64, t1323: f64, t6434: f64, t1385: f64, t6439: f64, t12021: f64, t6362: f64, t1375: f64, t1386: f64, t16022: f64, t16460: f64, t1843: f64, t3758: f64, t3882: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64, t6440: f64, t6461: f64, t20034: f64, t1390: f64, t6463: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t1297: f64, t1307: f64, t1388: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t193: f64, t19596: f64, t19599: f64, t19603: f64, t19631: f64, t19677: f64, t3918: f64, t5126: f64, t5160: f64, t5161: f64, t533: f64, t5356: f64, t571: f64, t6330: f64, t9780: f64, t9789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20038, t20040, t20044, t20048, t20051, t20060) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2337(t19804, t562, t1372, t6361, t225, t6435, t1323, t6434, t1385, t6439, t12021, t6362);
        let t20062 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2338(t1375, t1386, t16022, t16460, t1843, t20038, t20040, t20044, t20048, t20051, t20060, t3758, t3882, t5215, t5326, t5354, t568, t6440, t6461);
        let (t20063, t20067, t20075) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2339(t20034, t20062, t1390, t6463, t12044, t12048, t12057, t12059, t1297, t1307, t1388, t15898, t15911, t15916, t15917, t15923, t193, t19596, t19599, t19603, t19631, t19677, t3918, t5126, t5160, t5161, t533, t5356, t571, t6330, t9780, t9789);
    (t20038, t20040, t20044, t20048, t20051, t20060, t20063, t20067, t20075)
}
