//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta759 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2678;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2679;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2682;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta759<F: Float>(t14066: F, t545: F, t689: F, t869: F, t1398: F, t14141: F, t14143: F, t2434: F, t10049: F, t14145: F, t1882: F, t2482: F, t14230: F, t2782: F, t46456: F, t1385: F, t14155: F, t1432: F, t2470: F, t3999: F, t5710: F, t1892: F, t4056: F, t4086: F, t543: F, t10069: F, t14225: F, t1399: F, t4004: F, t47348: F, t47351: F, t47352: F, t47354: F, t47359: F, t49205: F, t5675: F, t5745: F, t820: F, t10013: F, t14224: F, t48073: F, t4100: F, t47364: F, t47369: F, t47375: F, t47379: F, t47381: F, t47387: F, t47389: F, t47391: F, t47395: F, t49213: F, t10136: F, t14114: F, t4104: F, t4118: F, t9990: F, t13805: F, t14122: F, t14127: F, t14193: F, t3924: F, t48015: F, t5755: F, t9995: F) -> (F, F, F, F, F, F, F, F) {
        let (t49252, t49256, t49260) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2678::<F>(t14066, t545, t689, t869, t1398, t14141, t14143, t2434, t10049, t14145, t1882, t2482);
        let (t49263, t49268, t49274, t49276, t49280) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2679::<F>(t14230, t2782, t46456, t1385, t14066, t14155, t1432, t2470, t3999, t5710, t1892, t4056);
        let t49293 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680::<F>(t2782, t4086, t49280, t543, t10069, t14225, t1399, t4004, t47348, t47351, t47352, t47354, t47359, t49205, t49268, t49274, t49276, t5675, t5745, t820);
        let t49310 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2681::<F>(t10013, t14224, t2782, t48073, t543, t4100, t47364, t47369, t47375, t47379, t47381, t47387, t47389, t47391, t47395);
        let (t49313, t49322, t49325, t49327) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2682::<F>(t2782, t4086, t49213, t543, t10136, t14114, t1882, t2482, t4104, t4118, t1892, t9990);
        let t49348 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2683::<F>(t1398, t2782, t4086, t543, t5710, t13805, t1399, t14122, t14127, t14193, t3924, t4004, t48015, t49313, t49322, t49325, t49327, t5745, t5755, t820, t9995);
    (t49252, t49256, t49260, t49263, t49280, t49293, t49310, t49348)
}
