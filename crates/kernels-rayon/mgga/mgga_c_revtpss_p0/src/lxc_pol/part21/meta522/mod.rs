//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2159;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta522(t1082: f64, t15837: f64, t3075: f64, t4975: f64, t4781: f64, t1071: f64, t3298: f64, t342: f64, t1089: f64, t4866: f64, t1024: f64, t1087: f64, t1090: f64, t12097: f64, t12154: f64, t16381: f64, t16390: f64, t16393: f64, t16396: f64, t16399: f64, t1647: f64, t1689: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t3292: f64, t3295: f64, t3322: f64, t4857: f64, t4964: f64, t4970: f64, t4984: f64, t4992: f64, t5012: f64, t989: f64, t1678: f64, t3151: f64, t3304: f64, t3302: f64, t4893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16402, t16405, t16406, t16409, t16410, t16414, t16423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2159(t1082, t15837, t3075, t4975, t4781, t1071, t3298, t342, t1089, t4866, t1024, t1087, t1090, t12097, t12154, t16381, t16390, t16393, t16396, t16399, t1647, t1689, t3204, t3223, t3278, t3287, t3292, t3295, t3322, t4857, t4964, t4970, t4984, t4992, t5012, t989);
        let (t16426, t16427, t16432) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2160(t1678, t3151, t3304, t3302, t4893);
    (t16402, t16405, t16406, t16409, t16410, t16414, t16423, t16426, t16427, t16432)
}
