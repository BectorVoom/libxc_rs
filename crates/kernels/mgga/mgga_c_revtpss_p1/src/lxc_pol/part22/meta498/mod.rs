//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2230;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta498<F: Float>(t1086: F, t4743: F, t1089: F, t15920: F, t16076: F, t12073: F, t1651: F, t1082: F, t16152: F, t15837: F, t3075: F, t4975: F, t4781: F, t1071: F, t3298: F, t342: F, t4866: F, t1024: F, t1087: F, t1090: F, t12097: F, t12154: F, t1647: F, t1689: F, t3204: F, t3223: F, t3278: F, t3287: F, t3292: F, t3295: F, t3322: F, t4857: F, t4964: F, t4970: F, t4984: F, t4992: F, t5012: F, t989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16381, t16390, t16393, t16396, t16399, t16402, t16405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2230::<F>(t1086, t4743, t1089, t15920, t16076, t12073, t1651, t1082, t16152, t15837, t3075, t4975);
        let (t16406, t16409, t16410, t16414, t16423) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2231::<F>(t16405, t4781, t1071, t3298, t342, t1089, t4866, t1024, t1087, t1090, t12097, t12154, t16381, t16390, t16393, t16396, t16399, t16402, t1647, t1689, t3204, t3223, t3278, t3287, t3292, t3295, t3322, t4857, t4964, t4970, t4984, t4992, t5012, t989);
    (t16381, t16390, t16393, t16396, t16399, t16402, t16405, t16406, t16409, t16410, t16414, t16423)
}
