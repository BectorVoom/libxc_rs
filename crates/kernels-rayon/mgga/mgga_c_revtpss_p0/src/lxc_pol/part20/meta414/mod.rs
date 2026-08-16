//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1529;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1530;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1532;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta414(t11692: f64, t11922: f64, t4899: f64, t1086: f64, t11213: f64, t3090: f64, t3057: f64, t3316: f64, t4891: f64, t3298: f64, t3059: f64, t3154: f64, t1045: f64, t2853: f64, t999: f64, t11774: f64, t127: f64, t3096: f64, t3128: f64, t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64, t11651: f64, t11659: f64, t11776: f64, t11866: f64, t11871: f64, t16025: f64, t3117: f64, t3120: f64, t372: f64, t42315: f64, t43029: f64, t43032: f64, t11858: f64, t15688: f64, t16102: f64, t3155: f64, t1020: f64, t12003: f64, t12077: f64, t15905: f64, t994: f64, t3075: f64, t11671: f64, t11865: f64, t11697: f64, t11710: f64, t3091: f64, t11725: f64, t828: f64, t11706: f64, t11660: f64, t2258: f64, t11779: f64, t3215: f64, t11231: f64, t11672: f64, t11678: f64, t11696: f64, t11698: f64, t11707: f64, t11811: f64, t11859: f64, t16095: f64, t2857: f64, t3092: f64, t3211: f64, t4892: f64, t3204: f64, t3230: f64, t225: f64, t42059: f64, t366: f64, t1053: f64, t11940: f64, t11675: f64, t11711: f64, t11666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43035, t43038, t43044, t43050, t43051) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1529(t11692, t11922, t4899, t1086, t11213, t3090, t3057, t3316, t4891, t3298, t3059, t3154);
        let (t43057, t43063, t43066, t43069) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1530(t1045, t2853, t999, t11774, t127, t3096, t3128, t11670, t11772, t3114, t11773, t11926);
        let t43074 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531(t11651, t11659, t11774, t11776, t11866, t11871, t16025, t3096, t3117, t3120, t372, t42315, t43029, t43032, t43035, t43038, t43044, t43050, t43051, t43057, t43063, t43066, t43069);
        let (t43082, t43085, t43091, t43105, t43116, t43121) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1532(t11858, t15688, t16102, t3155, t1020, t12003, t12077, t15905, t994, t3075, t3154, t11671, t11865);
        let t43148 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1533(t11697, t11710, t3091, t11725, t828, t11706, t11660, t2258, t11779, t3215, t11231, t11659, t11672, t11678, t11696, t11698, t11707, t11811, t11859, t16095, t2857, t3092, t3117, t3120, t3211, t43116, t43121, t4892, t999);
        let (t43151, t43154, t43155, t43161, t43169, t43172) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1534(t3204, t3230, t225, t42059, t366, t1053, t11940, t11675, t11711, t11666, t11710, t4899);
    (t43074, t43082, t43085, t43091, t43105, t43148, t43151, t43154, t43155, t43161, t43169, t43172)
}
