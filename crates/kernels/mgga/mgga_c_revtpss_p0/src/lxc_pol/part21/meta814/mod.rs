//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta814<F: Float>(t16226: F, t16229: F, t53405: F, t3075: F, t4910: F, t1043: F, t43051: F, t3059: F, t4900: F, t3230: F, t4857: F, t11817: F, t4858: F, t1028: F, t1042: F, t11875: F, t11927: F, t15604: F, t15691: F, t15700: F, t15780: F, t16222: F, t16223: F, t2858: F, t3117: F, t3155: F, t4186: F, t43044: F, t43050: F, t4781: F, t4837: F, t4872: F, t4893: F, t54166: F, t54267: F, t1045: F, t606: F, t3118: F, t1053: F, t15670: F, t11937: F, t15671: F, t11262: F, t3127: F, t4824: F, t11672: F, t11774: F, t11994: F, t1469: F, t15606: F, t15959: F, t16144: F, t16201: F, t3188: F, t3208: F, t372: F, t42425: F, t42675: F, t42795: F, t42798: F, t4806: F, t4825: F) -> (F, F, F, F, F, F) {
        let (t54348, t54360, t54365, t54370, t54384, t54387) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982::<F>(t16226, t16229, t53405, t3075, t4910, t1043, t43051, t3059, t4900, t3230, t4857, t11817, t4858);
        let t54389 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983::<F>(t54387, t1028, t1042, t1043, t11875, t11927, t15604, t15691, t15700, t15780, t16222, t16223, t16226, t2858, t3059, t3117, t3155, t4186, t43044, t43050, t4781, t4837, t4872, t4893, t54166, t54267, t54348, t54360, t54365, t54370, t54384);
        let (t54398, t54418) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984::<F>(t1045, t606, t3118, t1053, t15670, t11937, t15671, t11262, t3127, t4824, t11672, t11774, t11994, t1469, t15606, t15959, t16144, t16201, t3188, t3208, t372, t42425, t42675, t42795, t42798, t4806, t4825);
    (t54360, t54365, t54370, t54389, t54398, t54418)
}
