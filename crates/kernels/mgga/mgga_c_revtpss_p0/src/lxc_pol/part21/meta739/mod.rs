//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta739 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2592;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2593;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2594;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2595;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2596;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2597;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2598;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2599;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2600;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2601;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2602;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta739<F: Float>(t46289: F, t46291: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F, t14110: F, t47530: F, t1427: F, t1903: F, t22: F, t9647: F, t2453: F, t3908: F, t5711: F, t14296: F, t9303: F, t13738: F, t686: F, t72: F, t9680: F, t213: F, t556: F, t9656: F, t10146: F, t10147: F, t13743: F, t1424: F, t14299: F, t4071: F, t4076: F, t4078: F, t46353: F, t46356: F, t46359: F, t5715: F, t9651: F, t2439: F, t9640: F, t5718: F, t9292: F, t14274: F, t2435: F, t5599: F, t689: F, t13734: F, t1445: F, t10175: F, t14090: F, t14100: F, t9671: F, t1357: F, t14269: F, t1358: F, t14066: F, t212: F, t13747: F, t46368: F, t46369: F, t46378: F, t46381: F, t46385: F, t46388: F, t13746: F, t14085: F, t14104: F, t47520: F, t10069: F, t13731: F, t137: F, t14103: F, t47480: F, t9675: F, t14099: F, t9676: F, t14109: F, t9685: F, t4131: F, t47466: F, t47472: F, t47474: F, t47478: F, t47483: F, t47487: F, t5774: F, t9652: F, t5603: F, t9692: F, t9634: F, t1364: F, t14067: F, t786: F, t136: F, t2457: F, t9674: F, t46362: F, t47490: F, t47493: F, t47495: F, t47497: F, t47500: F, t47504: F, t47507: F, t47510: F, t47512: F, t47516: F, t47521: F, t9658: F, t14079: F, t10073: F, t3915: F, t5721: F, t9288: F, t3895: F, t5775: F, t47603: F, t9681: F, t14268: F, t14293: F, t9664: F, t13739: F, t47525: F, t47527: F, t47531: F, t47534: F, t47537: F, t47540: F, t1444: F, t2782: F, t4075: F, t4132: F, t9285: F, t13730: F, t1420: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47759, t47760, t47764, t47772, t47777, t47781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2592::<F>(t46289, t46291, t1892, t9646, t9648, t1904, t47567, t14110, t47530, t1427, t1903, t22, t9647);
        let (t47785, t47786, t47791, t47793, t47794) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2593::<F>(t2453, t3908, t5711, t14296, t9303, t13738, t686, t72, t9680, t213, t556, t1903, t9656);
        let t47798 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2594::<F>(t10146, t10147, t13743, t1424, t14299, t1903, t4071, t4076, t4078, t46353, t46356, t46359, t47764, t47772, t47777, t47781, t47785, t47786, t47791, t47793, t47794, t5715, t9651);
        let (t47800, t47802, t47806, t47808, t47811) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2595::<F>(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t4078, t5599, t689, t13734, t1445);
        let t47828 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2596::<F>(t10175, t14090, t14100, t9671, t1357, t14269, t689, t1358, t14066, t212, t13747, t4071, t46368, t46369, t46378, t46381, t46385, t46388, t47800, t47802, t47806, t47808, t47811);
        let (t47832, t47835, t47838, t47839, t47844) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2597::<F>(t13746, t686, t72, t9680, t14085, t2435, t14104, t47520, t10069, t13731, t137, t14103, t47480, t9675);
        let t47862 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2598::<F>(t47844, t14099, t2453, t9676, t14109, t9680, t9685, t1424, t4076, t4131, t47466, t47472, t47474, t47478, t47483, t47487, t47832, t47835, t47838, t47839, t5715, t5774, t9652);
        let (t47863, t47873, t47876, t47885) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2599::<F>(t5603, t9692, t1904, t689, t9634, t1364, t14067, t786, t136, t2457, t5774, t9674);
        let t47889 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2600::<F>(t47885, t1424, t1903, t46362, t47490, t47493, t47495, t47497, t47500, t47504, t47507, t47510, t47512, t47516, t47521, t47863, t47873, t47876, t9658);
        let (t47893, t47899, t47904, t47907, t47909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2601::<F>(t10175, t14079, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775, t14066, t213);
        let t47922 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2602::<F>(t14109, t47603, t9681, t14268, t3915, t686, t72, t14293, t9664, t13739, t1445, t4071, t47525, t47527, t47531, t47534, t47537, t47540, t47893, t47899, t47904, t47907, t47909);
        let (t47926, t47929, t47932, t47936) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2603::<F>(t1444, t2782, t4075, t556, t5774, t4132, t5599, t689, t14103, t9285, t9674, t13730, t1420);
    (t47759, t47760, t47794, t47798, t47828, t47862, t47889, t47922, t47926, t47929, t47932, t47936)
}
