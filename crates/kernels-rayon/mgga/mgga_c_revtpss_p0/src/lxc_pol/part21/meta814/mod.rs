//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta814(t16226: f64, t16229: f64, t53405: f64, t3075: f64, t4910: f64, t1043: f64, t43051: f64, t3059: f64, t4900: f64, t3230: f64, t4857: f64, t11817: f64, t4858: f64, t1028: f64, t1042: f64, t11875: f64, t11927: f64, t15604: f64, t15691: f64, t15700: f64, t15780: f64, t16222: f64, t16223: f64, t2858: f64, t3117: f64, t3155: f64, t4186: f64, t43044: f64, t43050: f64, t4781: f64, t4837: f64, t4872: f64, t4893: f64, t54166: f64, t54267: f64, t1045: f64, t606: f64, t3118: f64, t1053: f64, t15670: f64, t11937: f64, t15671: f64, t11262: f64, t3127: f64, t4824: f64, t11672: f64, t11774: f64, t11994: f64, t1469: f64, t15606: f64, t15959: f64, t16144: f64, t16201: f64, t3188: f64, t3208: f64, t372: f64, t42425: f64, t42675: f64, t42795: f64, t42798: f64, t4806: f64, t4825: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t54348, t54360, t54365, t54370, t54384, t54387) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982(t16226, t16229, t53405, t3075, t4910, t1043, t43051, t3059, t4900, t3230, t4857, t11817, t4858);
        let t54389 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983(t54387, t1028, t1042, t1043, t11875, t11927, t15604, t15691, t15700, t15780, t16222, t16223, t16226, t2858, t3059, t3117, t3155, t4186, t43044, t43050, t4781, t4837, t4872, t4893, t54166, t54267, t54348, t54360, t54365, t54370, t54384);
        let (t54398, t54418) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984(t1045, t606, t3118, t1053, t15670, t11937, t15671, t11262, t3127, t4824, t11672, t11774, t11994, t1469, t15606, t15959, t16144, t16201, t3188, t3208, t372, t42425, t42675, t42795, t42798, t4806, t4825);
    (t54360, t54365, t54370, t54389, t54398, t54418)
}
