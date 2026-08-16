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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1070(t73552: f64, t73576: f64, t22404: f64, t3920: f64, t1445: f64, t22445: f64, t689: f64, t13725: f64, t1904: f64, t2439: f64, t1364: f64, t22441: f64, t786: f64, t1424: f64, t14299: f64, t1444: f64, t22386: f64, t22415: f64, t4071: f64, t4076: f64, t46353: f64, t46356: f64, t46359: f64, t47764: f64, t47772: f64, t47777: f64, t47781: f64, t47784: f64, t47786: f64, t47791: f64, t47909: f64, t5728: f64, t5775: f64, t22446: f64, t2435: f64, t14079: f64, t14100: f64, t22433: f64, t46368: f64, t46369: f64, t46378: f64, t46385: f64, t46388: f64, t47800: f64, t47802: f64, t47805: f64, t47808: f64, t47811: f64, t47813: f64, t47816: f64, t47819: f64, t47825: f64, t47832: f64, t47834: f64, t3895: f64, t6919: f64, t10175: f64, t22399: f64, t13734: f64, t2453: f64, t3908: f64, t6889: f64, t22398: f64, t2470: f64, t3915: f64, t47466: f64, t47474: f64, t47478: f64, t47483: f64, t47487: f64, t47495: f64, t47497: f64, t47500: f64, t47837: f64, t47839: f64, t47844: f64, t47857: f64, t47860: f64, t47863: f64, t22452: f64, t9680: f64, t2782: f64, t556: f64, t6895: f64, t9656: f64, t22409: f64, t13730: f64, t1893: f64, t14268: f64, t1903: f64, t22390: f64, t4077: f64, t4078: f64, t46362: f64, t47504: f64, t47510: f64, t47512: f64, t47516: f64, t47521: f64, t47525: f64, t47527: f64, t47534: f64, t47873: f64, t47876: f64, t47885: f64, t47893: f64, t47899: f64, t3899: f64, t22449: f64, t136: f64, t2457: f64, t6918: f64, t9674: f64, t13999: f64, t22146: f64, t22145: f64, t48863: f64, t49137: f64, t124: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73578, t73587, t73590, t73593, t73598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3827(t73552, t73576, t22404, t3920, t1445, t22445, t689, t13725, t1904, t2439, t1364, t22441, t786);
        let t73614 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3828(t1424, t14299, t1444, t1904, t22386, t22415, t4071, t4076, t46353, t46356, t46359, t47764, t47772, t47777, t47781, t47784, t47786, t47791, t47909, t5728, t5775, t73587, t73590, t73593, t73598);
        let t73634 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3829(t22446, t2435, t14079, t14100, t22433, t4071, t46368, t46369, t46378, t46385, t46388, t47800, t47802, t47805, t47808, t47811, t47813, t47816, t47819, t47825, t47832, t47834);
        let (t73641, t73647, t73652, t73656, t73662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3830(t2439, t3895, t6919, t10175, t22399, t13734, t1904, t689, t2453, t3908, t6889, t22398, t2470, t3915);
        let t73664 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3831(t47466, t47474, t47478, t47483, t47487, t47495, t47497, t47500, t47837, t47839, t47844, t47857, t47860, t47863, t73641, t73647, t73652, t73656, t73662);
        let (t73666, t73671, t73673, t73676) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3832(t22452, t2470, t9680, t1444, t2782, t556, t6895, t9656, t22409, t2435, t13730, t1893);
        let t73700 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3833(t1424, t14268, t1903, t22390, t4076, t4077, t4078, t46362, t47504, t47510, t47512, t47516, t47521, t47525, t47527, t47534, t47873, t47876, t47885, t47893, t47899, t6895, t73666, t73671, t73673, t73676);
        let (t73705, t73707, t73712, t73726, t73729) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3834(t3899, t689, t6919, t22449, t2435, t136, t2457, t6918, t9674, t13999, t22146, t22145, t48863, t49137);
        let t73731 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3835(t124, t6861);
    (t73578, t73614, t73634, t73664, t73700, t73705, t73707, t73712, t73726, t73729, t73731)
}
