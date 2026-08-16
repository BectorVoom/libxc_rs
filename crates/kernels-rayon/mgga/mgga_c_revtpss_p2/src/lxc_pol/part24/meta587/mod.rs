//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta587 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1824;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1825;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1826;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1827;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1828;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1829;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1830;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1831;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1832;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1833;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1834;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta587(t543: f64, t92063: f64, t6843: f64, t124: f64, t1388: f64, t1390: f64, t1410: f64, t1868: f64, t22809: f64, t3944: f64, t4012: f64, t46730: f64, t48563: f64, t74264: f64, t74277: f64, t74279: f64, t74281: f64, t74290: f64, t800: f64, t828: f64, t85764: f64, t85778: f64, t85782: f64, t85791: f64, t85816: f64, t91870: f64, t91875: f64, t91942: f64, t13783: f64, t13790: f64, t1883: f64, t22046: f64, t22074: f64, t22079: f64, t22841: f64, t22852: f64, t3934: f64, t3936: f64, t46760: f64, t47248: f64, t48600: f64, t5671: f64, t5673: f64, t6862: f64, t6874: f64, t74299: f64, t74304: f64, t74322: f64, t74341: f64, t74358: f64, t74362: f64, t85548: f64, t85553: f64, t85609: f64, t85839: f64, t85865: f64, t91865: f64, t46800: f64, t46810: f64, t46817: f64, t46820: f64, t46824: f64, t46831: f64, t46840: f64, t48792: f64, t74429: f64, t74437: f64, t85873: f64, t85885: f64, t86061: f64, t86070: f64, t86074: f64, t86078: f64, t86080: f64, t4003: f64, t1872: f64, t4002: f64, t46885: f64, t48829: f64, t48833: f64, t48849: f64, t48853: f64, t48879: f64, t48909: f64, t6816: f64, t6836: f64, t6849: f64, t74485: f64, t74491: f64, t74493: f64, t74511: f64, t74522: f64, t86112: f64, t86124: f64, t9748: f64, t9942: f64, t91921: f64, t46478: f64, t47203: f64, t48947: f64, t49030: f64, t74585: f64, t86156: f64, t86165: f64, t86169: f64, t86183: f64, t86203: f64, t86208: f64, t86212: f64, t86220: f64, t86222: f64, t86226: f64, t86234: f64, t86236: f64, t1370: f64, t47337: f64, t49087: f64, t49090: f64, t49105: f64, t74638: f64, t74641: f64, t74677: f64, t74682: f64, t74711: f64, t74714: f64, t74717: f64, t86240: f64, t86244: f64, t86256: f64, t86260: f64, t86264: f64, t86274: f64, t91826: f64, t91882: f64, t91927: f64, t213: f64, t225: f64, t23043: f64, t46359: f64, t46368: f64, t46385: f64, t46388: f64, t47764: f64, t47772: f64, t47781: f64, t47786: f64, t47802: f64, t561: f64, t5715: f64, t73587: f64, t73593: f64, t73623: f64, t85475: f64, t1424: f64, t1903: f64, t23042: f64, t4076: f64, t47504: f64, t47863: f64, t47904: f64, t73641: f64, t73656: f64, t73662: f64, t73666: f64, t73673: f64, t73707: f64, t73712: f64, t85480: f64, t85484: f64, t85509: f64, t86285: f64, t86296: f64, t6918: f64, t47561: f64, t47920: f64, t47932: f64, t47938: f64, t49468: f64, t49474: f64, t74733: f64, t74757: f64, t74770: f64, t86300: f64, t86311: f64, t86314: f64, t86317: f64, t86346: f64, t10090: f64, t22912: f64, t4114: f64, t46476: f64, t47961: f64, t546: f64, t5767: f64, t74901: f64, t820: f64, t86374: f64, t86377: f64, t86381: f64, t86552: f64, t91922: f64, t14193: f64, t22005: f64, t22321: f64, t46515: f64, t46518: f64, t48036: f64, t6844: f64, t74999: f64, t75005: f64, t75021: f64, t75026: f64, t75068: f64, t85638: f64, t86468: f64, t1437: f64, t22009: f64, t47351: f64, t47395: f64, t5745: f64, t75145: f64, t75147: f64, t75176: f64, t75179: f64, t86563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92064, t92069, t92070, t92081) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1824(t543, t92063, t6843, t124, t1388, t1390, t1410, t1868, t22809, t3944, t4012, t46730, t48563, t74264, t74277, t74279, t74281, t74290, t800, t828, t85764, t85778, t85782, t85791, t85816, t91870, t91875, t91942);
        let t92123 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1825(t13783, t13790, t1883, t22046, t22074, t22079, t22841, t22852, t3934, t3936, t46760, t47248, t48600, t5671, t5673, t6862, t6874, t74299, t74304, t74322, t74341, t74358, t74362, t85548, t85553, t85609, t85839, t85865, t91865);
        let t92136 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1826(t46800, t46810, t46817, t46820, t46824, t46831, t46840, t48792, t74429, t74437, t85873, t85885, t86061, t86070, t86074, t86078, t86080);
        let (t92158, t92168) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1827(t4003, t92069, t1390, t1410, t1872, t22809, t3944, t4002, t46885, t48829, t48833, t48849, t48853, t48879, t48909, t6816, t6836, t6849, t74485, t74491, t74493, t74511, t74522, t800, t828, t86112, t86124, t9748, t9942);
        let (t92177, t92182, t92195) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1828(t4003, t91921, t46478, t1390, t4002, t47203, t48947, t49030, t74585, t828, t86156, t86165, t86169, t86183, t86203, t86208, t86212, t86220, t86222, t86226, t86234, t86236);
        let t92216 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1829(t124, t1370, t47337, t49087, t49090, t49105, t74638, t74641, t74677, t74682, t74711, t74714, t74717, t800, t86240, t86244, t86256, t86260, t86264, t86274, t91826);
        let (t92219, t92229) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1830(t91882, t91927, t92081, t92123, t92136, t92168, t92195, t92216, t213, t225, t23043, t46359, t46368, t46385, t46388, t47764, t47772, t47781, t47786, t47802, t561, t5715, t73587, t73593, t73623, t85475);
        let t92248 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1831(t1424, t1903, t23042, t4076, t47504, t47863, t47904, t73641, t73656, t73662, t73666, t73673, t73707, t73712, t85480, t85484, t85509, t86285, t86296);
        let t92267 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1832(t6918, t1424, t4076, t47561, t47920, t47932, t47938, t49468, t49474, t74733, t74757, t74770, t86300, t86311, t86314, t86317, t86346);
        let t92317 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1833(t10090, t1883, t213, t22912, t4114, t46476, t47961, t546, t5767, t74901, t820, t86374, t86377, t86381, t86552, t91922, t92177, t92182, t92219);
        let t92347 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1834(t14193, t22005, t22321, t46515, t46518, t48036, t6844, t74999, t75005, t75021, t75026, t75068, t820, t85638, t86468);
        let t92378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1835(t1437, t22009, t22321, t4114, t47351, t47395, t5745, t6862, t6874, t75145, t75147, t75176, t75179, t820, t86563, t91942, t92158);
    (t92064, t92070, t92229, t92248, t92267, t92317, t92347, t92378)
}
