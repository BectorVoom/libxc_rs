//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1070 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3827;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3828;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3829;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3830;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3831;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3832;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3833;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3834;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1070<F: Float>(t73552: F, t73576: F, t22404: F, t3920: F, t1445: F, t22445: F, t689: F, t13725: F, t1904: F, t2439: F, t1364: F, t22441: F, t786: F, t1424: F, t14299: F, t1444: F, t22386: F, t22415: F, t4071: F, t4076: F, t46353: F, t46356: F, t46359: F, t47764: F, t47772: F, t47777: F, t47781: F, t47784: F, t47786: F, t47791: F, t47909: F, t5728: F, t5775: F, t22446: F, t2435: F, t14079: F, t14100: F, t22433: F, t46368: F, t46369: F, t46378: F, t46385: F, t46388: F, t47800: F, t47802: F, t47805: F, t47808: F, t47811: F, t47813: F, t47816: F, t47819: F, t47825: F, t47832: F, t47834: F, t3895: F, t6919: F, t10175: F, t22399: F, t13734: F, t2453: F, t3908: F, t6889: F, t22398: F, t2470: F, t3915: F, t47466: F, t47474: F, t47478: F, t47483: F, t47487: F, t47495: F, t47497: F, t47500: F, t47837: F, t47839: F, t47844: F, t47857: F, t47860: F, t47863: F, t22452: F, t9680: F, t2782: F, t556: F, t6895: F, t9656: F, t22409: F, t13730: F, t1893: F, t14268: F, t1903: F, t22390: F, t4077: F, t4078: F, t46362: F, t47504: F, t47510: F, t47512: F, t47516: F, t47521: F, t47525: F, t47527: F, t47534: F, t47873: F, t47876: F, t47885: F, t47893: F, t47899: F, t3899: F, t22449: F, t136: F, t2457: F, t6918: F, t9674: F, t13999: F, t22146: F, t22145: F, t48863: F, t49137: F, t124: F, t6861: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73578, t73587, t73590, t73593, t73598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3827::<F>(t73552, t73576, t22404, t3920, t1445, t22445, t689, t13725, t1904, t2439, t1364, t22441, t786);
        let t73614 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3828::<F>(t1424, t14299, t1444, t1904, t22386, t22415, t4071, t4076, t46353, t46356, t46359, t47764, t47772, t47777, t47781, t47784, t47786, t47791, t47909, t5728, t5775, t73587, t73590, t73593, t73598);
        let t73634 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3829::<F>(t22446, t2435, t14079, t14100, t22433, t4071, t46368, t46369, t46378, t46385, t46388, t47800, t47802, t47805, t47808, t47811, t47813, t47816, t47819, t47825, t47832, t47834);
        let (t73641, t73647, t73652, t73656, t73662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3830::<F>(t2439, t3895, t6919, t10175, t22399, t13734, t1904, t689, t2453, t3908, t6889, t22398, t2470, t3915);
        let t73664 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3831::<F>(t47466, t47474, t47478, t47483, t47487, t47495, t47497, t47500, t47837, t47839, t47844, t47857, t47860, t47863, t73641, t73647, t73652, t73656, t73662);
        let (t73666, t73671, t73673, t73676) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3832::<F>(t22452, t2470, t9680, t1444, t2782, t556, t6895, t9656, t22409, t2435, t13730, t1893);
        let t73700 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3833::<F>(t1424, t14268, t1903, t22390, t4076, t4077, t4078, t46362, t47504, t47510, t47512, t47516, t47521, t47525, t47527, t47534, t47873, t47876, t47885, t47893, t47899, t6895, t73666, t73671, t73673, t73676);
        let (t73705, t73707, t73712, t73726, t73729) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3834::<F>(t3899, t689, t6919, t22449, t2435, t136, t2457, t6918, t9674, t13999, t22146, t22145, t48863, t49137);
        let t73731 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3835::<F>(t124, t6861);
    (t73578, t73614, t73634, t73664, t73700, t73705, t73707, t73712, t73726, t73729, t73731)
}
