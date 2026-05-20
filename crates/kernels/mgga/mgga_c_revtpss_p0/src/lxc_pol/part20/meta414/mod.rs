//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1529;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1530;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1532;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta414<F: Float>(t11692: F, t11922: F, t4899: F, t1086: F, t11213: F, t3090: F, t3057: F, t3316: F, t4891: F, t3298: F, t3059: F, t3154: F, t1045: F, t2853: F, t999: F, t11774: F, t127: F, t3096: F, t3128: F, t11670: F, t11772: F, t3114: F, t11773: F, t11926: F, t11651: F, t11659: F, t11776: F, t11866: F, t11871: F, t16025: F, t3117: F, t3120: F, t372: F, t42315: F, t43029: F, t43032: F, t11858: F, t15688: F, t16102: F, t3155: F, t1020: F, t12003: F, t12077: F, t15905: F, t994: F, t3075: F, t11671: F, t11865: F, t11697: F, t11710: F, t3091: F, t11725: F, t828: F, t11706: F, t11660: F, t2258: F, t11779: F, t3215: F, t11231: F, t11672: F, t11678: F, t11696: F, t11698: F, t11707: F, t11811: F, t11859: F, t16095: F, t2857: F, t3092: F, t3211: F, t4892: F, t3204: F, t3230: F, t225: F, t42059: F, t366: F, t1053: F, t11940: F, t11675: F, t11711: F, t11666: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43035, t43038, t43044, t43050, t43051) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1529::<F>(t11692, t11922, t4899, t1086, t11213, t3090, t3057, t3316, t4891, t3298, t3059, t3154);
        let (t43057, t43063, t43066, t43069) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1530::<F>(t1045, t2853, t999, t11774, t127, t3096, t3128, t11670, t11772, t3114, t11773, t11926);
        let t43074 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531::<F>(t11651, t11659, t11774, t11776, t11866, t11871, t16025, t3096, t3117, t3120, t372, t42315, t43029, t43032, t43035, t43038, t43044, t43050, t43051, t43057, t43063, t43066, t43069);
        let (t43082, t43085, t43091, t43105, t43116, t43121) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1532::<F>(t11858, t15688, t16102, t3155, t1020, t12003, t12077, t15905, t994, t3075, t3154, t11671, t11865);
        let t43148 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533::<F>(t11697, t11710, t3091, t11725, t828, t11706, t11660, t2258, t11779, t3215, t11231, t11659, t11672, t11678, t11696, t11698, t11707, t11811, t11859, t16095, t2857, t3092, t3117, t3120, t3211, t43116, t43121, t4892, t999);
        let (t43151, t43154, t43155, t43161, t43169, t43172) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1534::<F>(t3204, t3230, t225, t42059, t366, t1053, t11940, t11675, t11711, t11666, t11710, t4899);
    (t43074, t43082, t43085, t43091, t43105, t43148, t43151, t43154, t43155, t43161, t43169, t43172)
}
