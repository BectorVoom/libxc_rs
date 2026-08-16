//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1447;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1448;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1449;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1450;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1451;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1452;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1453;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta417(t19466: f64, t19476: f64, t1089: f64, t378: f64, t3302: f64, t357: f64, t4866: f64, t4893: f64, t1071: f64, t6299: f64, t1043: f64, t16560: f64, t19450: f64, t6258: f64, t3153: f64, t6305: f64, t4982: f64, t999: f64, t1024: f64, t1083: f64, t1087: f64, t11940: f64, t12122: f64, t12149: f64, t16544: f64, t16559: f64, t16566: f64, t19438: f64, t19443: f64, t19447: f64, t19453: f64, t19457: f64, t19463: f64, t3223: f64, t3287: f64, t4857: f64, t4954: f64, t4977: f64, t4988: f64, t4992: f64, t4996: f64, t5005: f64, t6368: f64, t4757: f64, t5004: f64, t3291: f64, t6244: f64, t1082: f64, t19399: f64, t1647: f64, t4980: f64, t3318: f64, t3304: f64, t16553: f64, t1093: f64, t11788: f64, t12160: f64, t15655: f64, t16502: f64, t16552: f64, t1685: f64, t3204: f64, t3299: f64, t3317: f64, t4964: f64, t4967: f64, t4981: f64, t4984: f64, t6235: f64, t6362: f64, t6371: f64, t6386: f64, t359: f64, t6343: f64, t1086: f64, t4995: f64, t4983: f64, t4998: f64, t1678: f64, t6271: f64, t3298: f64, t342: f64, t1090: f64, t12116: f64, t12127: f64, t16381: f64, t1689: f64, t1692: f64, t3278: f64, t4743: f64, t4970: f64, t4999: f64, t5009: f64, t5012: f64, t6375: f64, t6383: f64, t3316: f64, t73: f64, t4976: f64, t19414: f64, t1045: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19477, t19479, t19482, t19484, t19488, t19491) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1447(t19466, t19476, t1089, t378, t3302, t357, t4866, t4893, t1071, t6299, t1043, t16560);
        let (t19492, t19497, t19498, t19501) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1448(t19450, t19491, t1043, t6258, t1089, t3153, t6305);
        let t19508 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1449(t4982, t999, t19501, t1024, t1083, t1087, t11940, t12122, t12149, t16544, t16559, t16566, t19438, t19443, t19447, t19453, t19457, t19463, t19479, t19484, t19488, t19492, t19498, t3223, t3287, t4857, t4954, t4977, t4988, t4992, t4996, t5005, t6368);
        let (t19509, t19512, t19515, t19521, t19526, t19533) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1450(t4757, t5004, t3291, t6244, t1082, t19399, t4866, t4982, t4893, t1647, t4980, t1071, t6305);
        let t19554 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1451(t19533, t3318, t3304, t1043, t16553, t19450, t1093, t11788, t12160, t15655, t16502, t16544, t16552, t1685, t19509, t19512, t19515, t19521, t19526, t3204, t3223, t3299, t3317, t4857, t4964, t4967, t4977, t4981, t4984, t6235, t6362, t6371, t6386);
        let (t19557, t19566, t19569, t19572, t19573, t19576, t19579) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1452(t359, t6343, t999, t1086, t6235, t1647, t4995, t3153, t6299, t4983, t4998, t19482);
        let t19606 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1453(t19501, t19579, t1089, t1678, t4866, t3153, t6271, t4983, t4998, t3298, t342, t1024, t1087, t1090, t12116, t12122, t12127, t16381, t1647, t1689, t1692, t19557, t19566, t19569, t19573, t19576, t3278, t4743, t4857, t4954, t4970, t4981, t4984, t4996, t4999, t5009, t5012, t6375, t6383);
        let (t19608, t19611, t19612, t19617, t19622) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1454(t1678, t3316, t342, t6299, t73, t4976, t1082, t19414, t1045, t999, t6271, t3117);
    (t19477, t19497, t19501, t19508, t19554, t19572, t19606, t19608, t19611, t19612, t19617, t19622)
}
