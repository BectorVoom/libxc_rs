//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2230;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta498(t1086: f64, t4743: f64, t1089: f64, t15920: f64, t16076: f64, t12073: f64, t1651: f64, t1082: f64, t16152: f64, t15837: f64, t3075: f64, t4975: f64, t4781: f64, t1071: f64, t3298: f64, t342: f64, t4866: f64, t1024: f64, t1087: f64, t1090: f64, t12097: f64, t12154: f64, t1647: f64, t1689: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t3292: f64, t3295: f64, t3322: f64, t4857: f64, t4964: f64, t4970: f64, t4984: f64, t4992: f64, t5012: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16381, t16390, t16393, t16396, t16399, t16402, t16405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2230(t1086, t4743, t1089, t15920, t16076, t12073, t1651, t1082, t16152, t15837, t3075, t4975);
        let (t16406, t16409, t16410, t16414, t16423) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2231(t16405, t4781, t1071, t3298, t342, t1089, t4866, t1024, t1087, t1090, t12097, t12154, t16381, t16390, t16393, t16396, t16399, t16402, t1647, t1689, t3204, t3223, t3278, t3287, t3292, t3295, t3322, t4857, t4964, t4970, t4984, t4992, t5012, t989);
    (t16381, t16390, t16393, t16396, t16399, t16402, t16405, t16406, t16409, t16410, t16414, t16423)
}
