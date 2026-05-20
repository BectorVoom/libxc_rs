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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta592<F: Float>(t102218: F, t25878: F, t2470: F, t28844: F, t7284: F, t26292: F, t27884: F, t1904: F, t26354: F, t689: F, t26271: F, t27899: F, t1444: F, t1882: F, t25921: F, t25924: F, t26333: F, t26351: F, t27837: F, t28815: F, t28840: F, t543: F, t7295: F, t7301: F, t96284: F, t96287: F, t96289: F, t96292: F, t96294: F, t2435: F, t8099: F, t25904: F, t26231: F, t97802: F, t26234: F, t98041: F, t102244: F, t94674: F, t97700: F, t102268: F, t25930: F, t26335: F, t28863: F, t28890: F, t28911: F, t7292: F, t7917: F, t96296: F, t96298: F, t96371: F, t96374: F, t96378: F, t98362: F, t102165: F, t1445: F, t28824: F, t102274: F, t102100: F, t26069: F, t98380: F, t13730: F, t2098: F, t2782: F, t13747: F, t26282: F, t28850: F, t28899: F, t4131: F, t4132: F, t5728: F, t7296: F, t7511: F, t8085: F, t96380: F, t96382: F, t96398: F, t25899: F, t2439: F, t94391: F, t102234: F, t3916: F, t25895: F, t2097: F, t9990: F, t102115: F, t7289: F, t2103: F, t25933: F, t26304: F, t26305: F, t26371: F, t27868: F, t49393: F, t96401: F, t96403: F, t96410: F, t96412: F, t96423: F, t97737: F, t97933: F, t98053: F, t1426: F, t786: F, t8086: F, t3917: F, t14090: F, t26265: F, t14104: F, t96515: F, t13920: F, t28855: F, t49376: F, t7523: F, t96432: F, t96437: F, t97742: F, t97839: F, t97855: F, t98050: F, t98299: F, t5722: F, t96576: F, t28780: F, t94890: F, t28825: F, t14079: F, t98108: F, t98128: F, t98130: F, t98110: F, t98112: F, t98116: F, t98118: F, t98120: F, t98122: F, t98124: F, t98126: F, t98132: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102293, t102295, t102296, t102298, t102306, t102309) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1967::<F>(t102218, t25878, t2470, t28844, t7284, t26292, t27884, t1904, t26354, t689, t26271, t27899);
        let t102313 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1968::<F>(t102293, t102296, t102298, t102306, t102309, t1444, t1882, t25921, t25924, t26333, t26351, t27837, t28815, t28840, t543, t7295, t7301, t96284, t96287, t96289, t96292, t96294);
        let (t102315, t102341) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1969::<F>(t2435, t8099, t25904, t26231, t97802, t26234, t98041, t102244, t94674, t97700, t102268, t1882, t25921, t25930, t26335, t28863, t28890, t28911, t7292, t7917, t96296, t96298, t96371, t96374, t96378, t98362);
        let (t102346, t102361, t102363, t102364, t102367, t102372) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1970::<F>(t102165, t25904, t1445, t28824, t689, t102274, t25878, t102100, t26069, t26231, t98380, t13730, t2098, t2782);
        let t102374 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1971::<F>(t102346, t102361, t102363, t102364, t102367, t102372, t13747, t25921, t26282, t28850, t28899, t4131, t4132, t5728, t7295, t7296, t7511, t8085, t96380, t96382, t96398);
        let (t102378, t102385, t102386, t102394, t102396, t102397, t102404) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1972::<F>(t102315, t25899, t2439, t8099, t94391, t102234, t3916, t25895, t2097, t9990, t102115, t7289);
        let t102406 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1973::<F>(t102378, t102386, t102396, t102397, t102404, t1882, t2103, t25930, t25933, t26304, t26305, t26371, t27837, t27868, t49393, t96401, t96403, t96410, t96412, t96423, t97737, t97933, t98053);
        let (t102409, t102411, t102422, t102434, t102439) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1974::<F>(t26292, t27899, t102295, t7289, t1426, t786, t8086, t3917, t14090, t26265, t14104, t96515);
        let t102443 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1975::<F>(t102409, t102411, t102422, t102434, t102439, t13920, t2097, t25930, t26304, t27868, t28855, t49376, t543, t7295, t7301, t7523, t96432, t96437, t97742, t97839, t97855, t98050, t98299);
        let (t102453, t102458, t102462, t102465, t102480) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1976::<F>(t5722, t96576, t28780, t94890, t2435, t28825, t14079, t26265, t98108, t98128, t98130, t98110, t98112, t98116, t98118, t98120, t98122, t98124, t98126, t98132);
    (t102313, t102341, t102374, t102385, t102394, t102406, t102443, t102453, t102458, t102462, t102465, t102480)
}
