//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta613 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2107;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2108;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2109;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2110;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2111;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2112;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2113;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2114;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2115;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2116;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta613<F: Float>(t13829: F, t2661: F, t94550: F, t1873: F, t94519: F, t94520: F, t94527: F, t94537: F, t94540: F, t26004: F, t5690: F, t94514: F, t94523: F, t94526: F, t94530: F, t94534: F, t13951: F, t2018: F, t807: F, t94565: F, t25240: F, t3964: F, t5617: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94569: F, t94571: F, t98134: F, t98158: F, t98184: F, t98208: F, t98233: F, t98255: F, t543: F, t97870: F, t27857: F, t689: F, t25904: F, t786: F, t97961: F, t7286: F, t2439: F, t7925: F, t94391: F, t94383: F, t1444: F, t213: F, t225: F, t25921: F, t25924: F, t25930: F, t25931: F, t25961: F, t27837: F, t27846: F, t27858: F, t27902: F, t561: F, t7295: F, t94876: F, t98099: F, t98101: F, t98104: F, t25878: F, t98028: F, t94771: F, t97814: F, t1903: F, t1882: F, t2027: F, t2028: F, t25889: F, t25933: F, t26034: F, t26084: F, t27868: F, t49376: F, t545: F, t5775: F, t7296: F, t7301: F, t94823: F, t94880: F, t94882: F, t94884: F, t94887: F, t94891: F, t94895: F, t28002: F, t686: F, t72: F, t25895: F, t5722: F, t94748: F, t5675: F, t98067: F, t27968: F, t3920: F, t1445: F, t27985: F, t7242: F, t25898: F, t98040: F, t25901: F, t27989: F, t94921: F, t13747: F, t27980: F, t7279: F, t94898: F, t94902: F, t94904: F, t94802: F, t25899: F, t1904: F, t26079: F, t26081: F, t27909: F, t4003: F, t4132: F, t7298: F, t94906: F, t94909: F, t94911: F, t94914: F, t94917: F, t94919: F, t94922: F, t94931: F, t97909: F, t98050: F, t1450: F, t2014: F, t532: F, t97716: F, t97752: F, t97791: F, t97827: F, t97854: F, t97903: F, t97938: F, t97969: F, t97994: F, t98022: F, t98061: F, t98092: F, t1843: F, t25832: F, t651: F, t10416: F, t7742: F, t13435: F, t2322: F, t28063: F, t1907: F, t3889: F, t25082: F, t8717: F, t25188: F, t7935: F, t25802: F, t7934: F, t28167: F, t35070: F, t5627: F, t25081: F, t7897: F, t25083: F, t28020: F, t7315: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t98271 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2107::<F>(t13829, t2661, t94550, t1873, t94519, t94520, t94527, t94537, t94540, t26004, t5690, t94514, t94523, t94526, t94530, t94534);
        let t98287 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2108::<F>(t13951, t2018, t807, t94565, t25240, t3964, t5617, t94542, t94546, t94548, t94552, t94554, t94557, t94559, t94561, t94569, t94571);
        let (t98290, t98299, t98303) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2109::<F>(t98134, t98158, t98184, t98208, t98233, t98255, t98271, t98287, t543, t97870, t27857, t689);
        let t98318 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2110::<F>(t25904, t98303, t786, t97961, t7286, t2439, t7925, t94391, t94383, t1444, t213, t225, t25921, t25924, t25930, t25931, t25961, t27837, t27846, t27858, t27902, t561, t7295, t94876, t98099, t98101, t98104, t98290, t98299);
        let t98353 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2111::<F>(t25878, t98028, t94771, t97814, t1903, t25931, t1882, t2027, t2028, t25889, t25933, t26034, t26084, t27837, t27868, t49376, t543, t545, t5775, t7295, t7296, t7301, t94823, t94880, t94882, t94884, t94887, t94891, t94895, t98290);
        let (t98356, t98358, t98360, t98362, t98368, t98372, t98376) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2112::<F>(t28002, t686, t72, t25895, t5722, t94748, t1444, t5675, t98067, t27968, t3920, t1445, t27985, t689);
        let t98388 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2113::<F>(t5775, t689, t7242, t25898, t98040, t25901, t25878, t98356, t27989, t94921, t13747, t1882, t25930, t27980, t7279, t94898, t94902, t94904, t98358, t98360, t98362, t98368, t98372, t98376);
        let t98414 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2114::<F>(t27989, t94802, t25899, t98303, t1444, t1904, t25924, t26079, t26081, t27837, t27909, t28002, t4003, t4132, t7295, t7298, t94906, t94909, t94911, t94914, t94917, t94919, t94922, t94931, t97909, t98050);
        let t98421 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2115::<F>(t1450, t2014, t532, t97716, t97752, t97791, t97827, t97854, t97903, t97938, t97969, t97994, t98022, t98061, t98092, t98318, t98353, t98388, t98414);
        let (t98426, t98428, t98430, t98432, t98439) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2116::<F>(t1843, t25832, t651, t10416, t7742, t13435, t2322, t28063, t1907, t3889, t25082, t8717);
        let (t98440, t98442, t98449, t98452, t98455) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2117::<F>(t25188, t7935, t2014, t25802, t7934, t28167, t35070, t5627, t25081, t7897, t25083, t28020, t7315);
    (t98421, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452, t98455)
}
