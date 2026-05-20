//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2250;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2251;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta558<F: Float>(t17471: F, t5047: F, t1222: F, t1012: F, t13026: F, t16715: F, t16720: F, t5312: F, t1774: F, t3601: F, t3611: F, t3720: F, t12809: F, t12882: F, t12887: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F, t1263: F, t5245: F, t1122: F, t1042: F, t1234: F, t5390: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17474, t17475, t17476, t17479, t17482) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2250::<F>(t17471, t5047, t1222, t1012, t13026, t16715, t16720, t5312, t1774, t3601);
        let (t17483, t17484, t17493) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2251::<F>(t17482, t3611, t3720, t1222, t12809, t12882, t12887, t12893, t12895, t12900, t12902, t12905, t17474, t17476, t17479);
        let (t17500, t17501, t17502, t17505) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2252::<F>(t1263, t5245, t1122, t1042, t1234, t5390);
    (t17475, t17482, t17483, t17484, t17493, t17500, t17501, t17502, t17505)
}
