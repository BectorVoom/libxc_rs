//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta759 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2678;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2679;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2682;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta759(t14066: f64, t545: f64, t689: f64, t869: f64, t1398: f64, t14141: f64, t14143: f64, t2434: f64, t10049: f64, t14145: f64, t1882: f64, t2482: f64, t14230: f64, t2782: f64, t46456: f64, t1385: f64, t14155: f64, t1432: f64, t2470: f64, t3999: f64, t5710: f64, t1892: f64, t4056: f64, t4086: f64, t543: f64, t10069: f64, t14225: f64, t1399: f64, t4004: f64, t47348: f64, t47351: f64, t47352: f64, t47354: f64, t47359: f64, t49205: f64, t5675: f64, t5745: f64, t820: f64, t10013: f64, t14224: f64, t48073: f64, t4100: f64, t47364: f64, t47369: f64, t47375: f64, t47379: f64, t47381: f64, t47387: f64, t47389: f64, t47391: f64, t47395: f64, t49213: f64, t10136: f64, t14114: f64, t4104: f64, t4118: f64, t9990: f64, t13805: f64, t14122: f64, t14127: f64, t14193: f64, t3924: f64, t48015: f64, t5755: f64, t9995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49252, t49256, t49260) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2678(t14066, t545, t689, t869, t1398, t14141, t14143, t2434, t10049, t14145, t1882, t2482);
        let (t49263, t49268, t49274, t49276, t49280) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2679(t14230, t2782, t46456, t1385, t14066, t14155, t1432, t2470, t3999, t5710, t1892, t4056);
        let t49293 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680(t2782, t4086, t49280, t543, t10069, t14225, t1399, t4004, t47348, t47351, t47352, t47354, t47359, t49205, t49268, t49274, t49276, t5675, t5745, t820);
        let t49310 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681(t10013, t14224, t2782, t48073, t543, t4100, t47364, t47369, t47375, t47379, t47381, t47387, t47389, t47391, t47395);
        let (t49313, t49322, t49325, t49327) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2682(t2782, t4086, t49213, t543, t10136, t14114, t1882, t2482, t4104, t4118, t1892, t9990);
        let t49348 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683(t1398, t2782, t4086, t543, t5710, t13805, t1399, t14122, t14127, t14193, t3924, t4004, t48015, t49313, t49322, t49325, t49327, t5745, t5755, t820, t9995);
    (t49252, t49256, t49260, t49263, t49280, t49293, t49310, t49348)
}
