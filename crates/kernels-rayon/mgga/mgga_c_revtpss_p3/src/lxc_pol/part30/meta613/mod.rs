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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta613(t13829: f64, t2661: f64, t94550: f64, t1873: f64, t94519: f64, t94520: f64, t94527: f64, t94537: f64, t94540: f64, t26004: f64, t5690: f64, t94514: f64, t94523: f64, t94526: f64, t94530: f64, t94534: f64, t13951: f64, t2018: f64, t807: f64, t94565: f64, t25240: f64, t3964: f64, t5617: f64, t94542: f64, t94546: f64, t94548: f64, t94552: f64, t94554: f64, t94557: f64, t94559: f64, t94561: f64, t94569: f64, t94571: f64, t98134: f64, t98158: f64, t98184: f64, t98208: f64, t98233: f64, t98255: f64, t543: f64, t97870: f64, t27857: f64, t689: f64, t25904: f64, t786: f64, t97961: f64, t7286: f64, t2439: f64, t7925: f64, t94391: f64, t94383: f64, t1444: f64, t213: f64, t225: f64, t25921: f64, t25924: f64, t25930: f64, t25931: f64, t25961: f64, t27837: f64, t27846: f64, t27858: f64, t27902: f64, t561: f64, t7295: f64, t94876: f64, t98099: f64, t98101: f64, t98104: f64, t25878: f64, t98028: f64, t94771: f64, t97814: f64, t1903: f64, t1882: f64, t2027: f64, t2028: f64, t25889: f64, t25933: f64, t26034: f64, t26084: f64, t27868: f64, t49376: f64, t545: f64, t5775: f64, t7296: f64, t7301: f64, t94823: f64, t94880: f64, t94882: f64, t94884: f64, t94887: f64, t94891: f64, t94895: f64, t28002: f64, t686: f64, t72: f64, t25895: f64, t5722: f64, t94748: f64, t5675: f64, t98067: f64, t27968: f64, t3920: f64, t1445: f64, t27985: f64, t7242: f64, t25898: f64, t98040: f64, t25901: f64, t27989: f64, t94921: f64, t13747: f64, t27980: f64, t7279: f64, t94898: f64, t94902: f64, t94904: f64, t94802: f64, t25899: f64, t1904: f64, t26079: f64, t26081: f64, t27909: f64, t4003: f64, t4132: f64, t7298: f64, t94906: f64, t94909: f64, t94911: f64, t94914: f64, t94917: f64, t94919: f64, t94922: f64, t94931: f64, t97909: f64, t98050: f64, t1450: f64, t2014: f64, t532: f64, t97716: f64, t97752: f64, t97791: f64, t97827: f64, t97854: f64, t97903: f64, t97938: f64, t97969: f64, t97994: f64, t98022: f64, t98061: f64, t98092: f64, t1843: f64, t25832: f64, t651: f64, t10416: f64, t7742: f64, t13435: f64, t2322: f64, t28063: f64, t1907: f64, t3889: f64, t25082: f64, t8717: f64, t25188: f64, t7935: f64, t25802: f64, t7934: f64, t28167: f64, t35070: f64, t5627: f64, t25081: f64, t7897: f64, t25083: f64, t28020: f64, t7315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t98271 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2107(t13829, t2661, t94550, t1873, t94519, t94520, t94527, t94537, t94540, t26004, t5690, t94514, t94523, t94526, t94530, t94534);
        let t98287 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2108(t13951, t2018, t807, t94565, t25240, t3964, t5617, t94542, t94546, t94548, t94552, t94554, t94557, t94559, t94561, t94569, t94571);
        let (t98290, t98299, t98303) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2109(t98134, t98158, t98184, t98208, t98233, t98255, t98271, t98287, t543, t97870, t27857, t689);
        let t98318 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2110(t25904, t98303, t786, t97961, t7286, t2439, t7925, t94391, t94383, t1444, t213, t225, t25921, t25924, t25930, t25931, t25961, t27837, t27846, t27858, t27902, t561, t7295, t94876, t98099, t98101, t98104, t98290, t98299);
        let t98353 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2111(t25878, t98028, t94771, t97814, t1903, t25931, t1882, t2027, t2028, t25889, t25933, t26034, t26084, t27837, t27868, t49376, t543, t545, t5775, t7295, t7296, t7301, t94823, t94880, t94882, t94884, t94887, t94891, t94895, t98290);
        let (t98356, t98358, t98360, t98362, t98368, t98372, t98376) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2112(t28002, t686, t72, t25895, t5722, t94748, t1444, t5675, t98067, t27968, t3920, t1445, t27985, t689);
        let t98388 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2113(t5775, t689, t7242, t25898, t98040, t25901, t25878, t98356, t27989, t94921, t13747, t1882, t25930, t27980, t7279, t94898, t94902, t94904, t98358, t98360, t98362, t98368, t98372, t98376);
        let t98414 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2114(t27989, t94802, t25899, t98303, t1444, t1904, t25924, t26079, t26081, t27837, t27909, t28002, t4003, t4132, t7295, t7298, t94906, t94909, t94911, t94914, t94917, t94919, t94922, t94931, t97909, t98050);
        let t98421 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2115(t1450, t2014, t532, t97716, t97752, t97791, t97827, t97854, t97903, t97938, t97969, t97994, t98022, t98061, t98092, t98318, t98353, t98388, t98414);
        let (t98426, t98428, t98430, t98432, t98439) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2116(t1843, t25832, t651, t10416, t7742, t13435, t2322, t28063, t1907, t3889, t25082, t8717);
        let (t98440, t98442, t98449, t98452, t98455) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2117(t25188, t7935, t2014, t25802, t7934, t28167, t35070, t5627, t25081, t7897, t25083, t28020, t7315);
    (t98421, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452, t98455)
}
