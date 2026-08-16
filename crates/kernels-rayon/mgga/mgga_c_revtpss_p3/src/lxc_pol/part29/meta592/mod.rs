//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta592 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1969;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1970;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1971;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1972;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1973;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1974;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1975;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta592(t102218: f64, t25878: f64, t2470: f64, t28844: f64, t7284: f64, t26292: f64, t27884: f64, t1904: f64, t26354: f64, t689: f64, t26271: f64, t27899: f64, t1444: f64, t1882: f64, t25921: f64, t25924: f64, t26333: f64, t26351: f64, t27837: f64, t28815: f64, t28840: f64, t543: f64, t7295: f64, t7301: f64, t96284: f64, t96287: f64, t96289: f64, t96292: f64, t96294: f64, t2435: f64, t8099: f64, t25904: f64, t26231: f64, t97802: f64, t26234: f64, t98041: f64, t102244: f64, t94674: f64, t97700: f64, t102268: f64, t25930: f64, t26335: f64, t28863: f64, t28890: f64, t28911: f64, t7292: f64, t7917: f64, t96296: f64, t96298: f64, t96371: f64, t96374: f64, t96378: f64, t98362: f64, t102165: f64, t1445: f64, t28824: f64, t102274: f64, t102100: f64, t26069: f64, t98380: f64, t13730: f64, t2098: f64, t2782: f64, t13747: f64, t26282: f64, t28850: f64, t28899: f64, t4131: f64, t4132: f64, t5728: f64, t7296: f64, t7511: f64, t8085: f64, t96380: f64, t96382: f64, t96398: f64, t25899: f64, t2439: f64, t94391: f64, t102234: f64, t3916: f64, t25895: f64, t2097: f64, t9990: f64, t102115: f64, t7289: f64, t2103: f64, t25933: f64, t26304: f64, t26305: f64, t26371: f64, t27868: f64, t49393: f64, t96401: f64, t96403: f64, t96410: f64, t96412: f64, t96423: f64, t97737: f64, t97933: f64, t98053: f64, t1426: f64, t786: f64, t8086: f64, t3917: f64, t14090: f64, t26265: f64, t14104: f64, t96515: f64, t13920: f64, t28855: f64, t49376: f64, t7523: f64, t96432: f64, t96437: f64, t97742: f64, t97839: f64, t97855: f64, t98050: f64, t98299: f64, t5722: f64, t96576: f64, t28780: f64, t94890: f64, t28825: f64, t14079: f64, t98108: f64, t98128: f64, t98130: f64, t98110: f64, t98112: f64, t98116: f64, t98118: f64, t98120: f64, t98122: f64, t98124: f64, t98126: f64, t98132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102293, t102295, t102296, t102298, t102306, t102309) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1967(t102218, t25878, t2470, t28844, t7284, t26292, t27884, t1904, t26354, t689, t26271, t27899);
        let t102313 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1968(t102293, t102296, t102298, t102306, t102309, t1444, t1882, t25921, t25924, t26333, t26351, t27837, t28815, t28840, t543, t7295, t7301, t96284, t96287, t96289, t96292, t96294);
        let (t102315, t102341) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1969(t2435, t8099, t25904, t26231, t97802, t26234, t98041, t102244, t94674, t97700, t102268, t1882, t25921, t25930, t26335, t28863, t28890, t28911, t7292, t7917, t96296, t96298, t96371, t96374, t96378, t98362);
        let (t102346, t102361, t102363, t102364, t102367, t102372) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1970(t102165, t25904, t1445, t28824, t689, t102274, t25878, t102100, t26069, t26231, t98380, t13730, t2098, t2782);
        let t102374 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1971(t102346, t102361, t102363, t102364, t102367, t102372, t13747, t25921, t26282, t28850, t28899, t4131, t4132, t5728, t7295, t7296, t7511, t8085, t96380, t96382, t96398);
        let (t102378, t102385, t102386, t102394, t102396, t102397, t102404) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1972(t102315, t25899, t2439, t8099, t94391, t102234, t3916, t25895, t2097, t9990, t102115, t7289);
        let t102406 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1973(t102378, t102386, t102396, t102397, t102404, t1882, t2103, t25930, t25933, t26304, t26305, t26371, t27837, t27868, t49393, t96401, t96403, t96410, t96412, t96423, t97737, t97933, t98053);
        let (t102409, t102411, t102422, t102434, t102439) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1974(t26292, t27899, t102295, t7289, t1426, t786, t8086, t3917, t14090, t26265, t14104, t96515);
        let t102443 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1975(t102409, t102411, t102422, t102434, t102439, t13920, t2097, t25930, t26304, t27868, t28855, t49376, t543, t7295, t7301, t7523, t96432, t96437, t97742, t97839, t97855, t98050, t98299);
        let (t102453, t102458, t102462, t102465, t102480) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1976(t5722, t96576, t28780, t94890, t2435, t28825, t14079, t26265, t98108, t98128, t98130, t98110, t98112, t98116, t98118, t98120, t98122, t98124, t98126, t98132);
    (t102313, t102341, t102374, t102385, t102394, t102406, t102443, t102453, t102458, t102462, t102465, t102480)
}
