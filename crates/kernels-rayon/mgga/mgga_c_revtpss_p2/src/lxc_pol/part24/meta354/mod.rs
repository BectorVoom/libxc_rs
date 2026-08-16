//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1219;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta354(t225: f64, t23959: f64, t366: f64, t1651: f64, t6258: f64, t247: f64, t3116: f64, t1066: f64, t23474: f64, t11853: f64, t23470: f64, t23499: f64, t4919: f64, t1011: f64, t1063: f64, t11774: f64, t11972: f64, t15862: f64, t1675: f64, t19901: f64, t19908: f64, t19913: f64, t19921: f64, t19968: f64, t19977: f64, t23931: f64, t23936: f64, t23939: f64, t23945: f64, t375: f64, t4834: f64, t4837: f64, t4892: f64, t4899: f64, t6323: f64, t6327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23960, t23961, t23964) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1219(t225, t23959, t366, t1651, t6258);
        let (t23966, t23976, t23980, t23988) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1220(t23964, t247, t3116, t1066, t23474, t11853, t23470, t23499, t4919, t1011, t1063, t11774, t11972, t15862, t1675, t19901, t19908, t19913, t19921, t19968, t19977, t23931, t23936, t23939, t23945, t23961, t375, t4834, t4837, t4892, t4899, t6323, t6327);
    (t23960, t23961, t23964, t23966, t23976, t23980, t23988)
}
