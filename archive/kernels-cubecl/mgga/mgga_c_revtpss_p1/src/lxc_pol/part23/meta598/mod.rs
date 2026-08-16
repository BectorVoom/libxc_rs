//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2243;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2244;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2245;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta598<F: Float>(t225: F, t23959: F, t366: F, t1651: F, t6258: F, t247: F, t3116: F, t1066: F, t23474: F, t11853: F, t23470: F, t23499: F, t4919: F, t1011: F, t1063: F, t11774: F, t11972: F, t15862: F, t1675: F, t19901: F, t19908: F, t19913: F, t19921: F, t19968: F, t19977: F, t23931: F, t23936: F, t23939: F, t23945: F, t375: F, t4834: F, t4837: F, t4892: F, t4899: F, t6323: F, t6327: F, t1668: F, t1045: F, t3117: F, t6299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23960, t23961, t23964) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2243::<F>(t225, t23959, t366, t1651, t6258);
        let (t23966, t23976, t23980, t23984, t23988) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2244::<F>(t23964, t247, t3116, t1066, t23474, t11853, t23470, t23499, t4919, t1011, t1063, t11774, t11972, t15862, t1675, t19901, t19908, t19913, t19921, t19968, t19977, t23931, t23936, t23939, t23945, t23961, t375, t4834, t4837, t4892, t4899, t6323, t6327);
        let t23992 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2245::<F>(t1668, t6258);
        let (t23993, t23994, t23997) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2246::<F>(t1045, t23992, t3117, t1651, t6299);
    (t23960, t23961, t23964, t23966, t23976, t23980, t23984, t23988, t23992, t23993, t23994, t23997)
}
