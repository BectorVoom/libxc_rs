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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta739(t46289: f64, t46291: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64, t47567: f64, t14110: f64, t47530: f64, t1427: f64, t1903: f64, t22: f64, t9647: f64, t2453: f64, t3908: f64, t5711: f64, t14296: f64, t9303: f64, t13738: f64, t686: f64, t72: f64, t9680: f64, t213: f64, t556: f64, t9656: f64, t10146: f64, t10147: f64, t13743: f64, t1424: f64, t14299: f64, t4071: f64, t4076: f64, t4078: f64, t46353: f64, t46356: f64, t46359: f64, t5715: f64, t9651: f64, t2439: f64, t9640: f64, t5718: f64, t9292: f64, t14274: f64, t2435: f64, t5599: f64, t689: f64, t13734: f64, t1445: f64, t10175: f64, t14090: f64, t14100: f64, t9671: f64, t1357: f64, t14269: f64, t1358: f64, t14066: f64, t212: f64, t13747: f64, t46368: f64, t46369: f64, t46378: f64, t46381: f64, t46385: f64, t46388: f64, t13746: f64, t14085: f64, t14104: f64, t47520: f64, t10069: f64, t13731: f64, t137: f64, t14103: f64, t47480: f64, t9675: f64, t14099: f64, t9676: f64, t14109: f64, t9685: f64, t4131: f64, t47466: f64, t47472: f64, t47474: f64, t47478: f64, t47483: f64, t47487: f64, t5774: f64, t9652: f64, t5603: f64, t9692: f64, t9634: f64, t1364: f64, t14067: f64, t786: f64, t136: f64, t2457: f64, t9674: f64, t46362: f64, t47490: f64, t47493: f64, t47495: f64, t47497: f64, t47500: f64, t47504: f64, t47507: f64, t47510: f64, t47512: f64, t47516: f64, t47521: f64, t9658: f64, t14079: f64, t10073: f64, t3915: f64, t5721: f64, t9288: f64, t3895: f64, t5775: f64, t47603: f64, t9681: f64, t14268: f64, t14293: f64, t9664: f64, t13739: f64, t47525: f64, t47527: f64, t47531: f64, t47534: f64, t47537: f64, t47540: f64, t1444: f64, t2782: f64, t4075: f64, t4132: f64, t9285: f64, t13730: f64, t1420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47759, t47760, t47764, t47772, t47777, t47781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2592(t46289, t46291, t1892, t9646, t9648, t1904, t47567, t14110, t47530, t1427, t1903, t22, t9647);
        let (t47785, t47786, t47791, t47793, t47794) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2593(t2453, t3908, t5711, t14296, t9303, t13738, t686, t72, t9680, t213, t556, t1903, t9656);
        let t47798 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2594(t10146, t10147, t13743, t1424, t14299, t1903, t4071, t4076, t4078, t46353, t46356, t46359, t47764, t47772, t47777, t47781, t47785, t47786, t47791, t47793, t47794, t5715, t9651);
        let (t47800, t47802, t47806, t47808, t47811) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2595(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t4078, t5599, t689, t13734, t1445);
        let t47828 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2596(t10175, t14090, t14100, t9671, t1357, t14269, t689, t1358, t14066, t212, t13747, t4071, t46368, t46369, t46378, t46381, t46385, t46388, t47800, t47802, t47806, t47808, t47811);
        let (t47832, t47835, t47838, t47839, t47844) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2597(t13746, t686, t72, t9680, t14085, t2435, t14104, t47520, t10069, t13731, t137, t14103, t47480, t9675);
        let t47862 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2598(t47844, t14099, t2453, t9676, t14109, t9680, t9685, t1424, t4076, t4131, t47466, t47472, t47474, t47478, t47483, t47487, t47832, t47835, t47838, t47839, t5715, t5774, t9652);
        let (t47863, t47873, t47876, t47885) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2599(t5603, t9692, t1904, t689, t9634, t1364, t14067, t786, t136, t2457, t5774, t9674);
        let t47889 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2600(t47885, t1424, t1903, t46362, t47490, t47493, t47495, t47497, t47500, t47504, t47507, t47510, t47512, t47516, t47521, t47863, t47873, t47876, t9658);
        let (t47893, t47899, t47904, t47907, t47909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2601(t10175, t14079, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775, t14066, t213);
        let t47922 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2602(t14109, t47603, t9681, t14268, t3915, t686, t72, t14293, t9664, t13739, t1445, t4071, t47525, t47527, t47531, t47534, t47537, t47540, t47893, t47899, t47904, t47907, t47909);
        let (t47926, t47929, t47932, t47936) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2603(t1444, t2782, t4075, t556, t5774, t4132, t5599, t689, t14103, t9285, t9674, t13730, t1420);
    (t47759, t47760, t47794, t47798, t47828, t47862, t47889, t47922, t47926, t47929, t47932, t47936)
}
