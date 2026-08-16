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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta587<F: Float>(t543: F, t92063: F, t6843: F, t124: F, t1388: F, t1390: F, t1410: F, t1868: F, t22809: F, t3944: F, t4012: F, t46730: F, t48563: F, t74264: F, t74277: F, t74279: F, t74281: F, t74290: F, t800: F, t828: F, t85764: F, t85778: F, t85782: F, t85791: F, t85816: F, t91870: F, t91875: F, t91942: F, t13783: F, t13790: F, t1883: F, t22046: F, t22074: F, t22079: F, t22841: F, t22852: F, t3934: F, t3936: F, t46760: F, t47248: F, t48600: F, t5671: F, t5673: F, t6862: F, t6874: F, t74299: F, t74304: F, t74322: F, t74341: F, t74358: F, t74362: F, t85548: F, t85553: F, t85609: F, t85839: F, t85865: F, t91865: F, t46800: F, t46810: F, t46817: F, t46820: F, t46824: F, t46831: F, t46840: F, t48792: F, t74429: F, t74437: F, t85873: F, t85885: F, t86061: F, t86070: F, t86074: F, t86078: F, t86080: F, t4003: F, t1872: F, t4002: F, t46885: F, t48829: F, t48833: F, t48849: F, t48853: F, t48879: F, t48909: F, t6816: F, t6836: F, t6849: F, t74485: F, t74491: F, t74493: F, t74511: F, t74522: F, t86112: F, t86124: F, t9748: F, t9942: F, t91921: F, t46478: F, t47203: F, t48947: F, t49030: F, t74585: F, t86156: F, t86165: F, t86169: F, t86183: F, t86203: F, t86208: F, t86212: F, t86220: F, t86222: F, t86226: F, t86234: F, t86236: F, t1370: F, t47337: F, t49087: F, t49090: F, t49105: F, t74638: F, t74641: F, t74677: F, t74682: F, t74711: F, t74714: F, t74717: F, t86240: F, t86244: F, t86256: F, t86260: F, t86264: F, t86274: F, t91826: F, t91882: F, t91927: F, t213: F, t225: F, t23043: F, t46359: F, t46368: F, t46385: F, t46388: F, t47764: F, t47772: F, t47781: F, t47786: F, t47802: F, t561: F, t5715: F, t73587: F, t73593: F, t73623: F, t85475: F, t1424: F, t1903: F, t23042: F, t4076: F, t47504: F, t47863: F, t47904: F, t73641: F, t73656: F, t73662: F, t73666: F, t73673: F, t73707: F, t73712: F, t85480: F, t85484: F, t85509: F, t86285: F, t86296: F, t6918: F, t47561: F, t47920: F, t47932: F, t47938: F, t49468: F, t49474: F, t74733: F, t74757: F, t74770: F, t86300: F, t86311: F, t86314: F, t86317: F, t86346: F, t10090: F, t22912: F, t4114: F, t46476: F, t47961: F, t546: F, t5767: F, t74901: F, t820: F, t86374: F, t86377: F, t86381: F, t86552: F, t91922: F, t14193: F, t22005: F, t22321: F, t46515: F, t46518: F, t48036: F, t6844: F, t74999: F, t75005: F, t75021: F, t75026: F, t75068: F, t85638: F, t86468: F, t1437: F, t22009: F, t47351: F, t47395: F, t5745: F, t75145: F, t75147: F, t75176: F, t75179: F, t86563: F) -> (F, F, F, F, F, F, F, F) {
        let (t92064, t92069, t92070, t92081) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1824::<F>(t543, t92063, t6843, t124, t1388, t1390, t1410, t1868, t22809, t3944, t4012, t46730, t48563, t74264, t74277, t74279, t74281, t74290, t800, t828, t85764, t85778, t85782, t85791, t85816, t91870, t91875, t91942);
        let t92123 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1825::<F>(t13783, t13790, t1883, t22046, t22074, t22079, t22841, t22852, t3934, t3936, t46760, t47248, t48600, t5671, t5673, t6862, t6874, t74299, t74304, t74322, t74341, t74358, t74362, t85548, t85553, t85609, t85839, t85865, t91865);
        let t92136 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1826::<F>(t46800, t46810, t46817, t46820, t46824, t46831, t46840, t48792, t74429, t74437, t85873, t85885, t86061, t86070, t86074, t86078, t86080);
        let (t92158, t92168) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1827::<F>(t4003, t92069, t1390, t1410, t1872, t22809, t3944, t4002, t46885, t48829, t48833, t48849, t48853, t48879, t48909, t6816, t6836, t6849, t74485, t74491, t74493, t74511, t74522, t800, t828, t86112, t86124, t9748, t9942);
        let (t92177, t92182, t92195) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1828::<F>(t4003, t91921, t46478, t1390, t4002, t47203, t48947, t49030, t74585, t828, t86156, t86165, t86169, t86183, t86203, t86208, t86212, t86220, t86222, t86226, t86234, t86236);
        let t92216 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1829::<F>(t124, t1370, t47337, t49087, t49090, t49105, t74638, t74641, t74677, t74682, t74711, t74714, t74717, t800, t86240, t86244, t86256, t86260, t86264, t86274, t91826);
        let (t92219, t92229) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1830::<F>(t91882, t91927, t92081, t92123, t92136, t92168, t92195, t92216, t213, t225, t23043, t46359, t46368, t46385, t46388, t47764, t47772, t47781, t47786, t47802, t561, t5715, t73587, t73593, t73623, t85475);
        let t92248 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1831::<F>(t1424, t1903, t23042, t4076, t47504, t47863, t47904, t73641, t73656, t73662, t73666, t73673, t73707, t73712, t85480, t85484, t85509, t86285, t86296);
        let t92267 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1832::<F>(t6918, t1424, t4076, t47561, t47920, t47932, t47938, t49468, t49474, t74733, t74757, t74770, t86300, t86311, t86314, t86317, t86346);
        let t92317 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1833::<F>(t10090, t1883, t213, t22912, t4114, t46476, t47961, t546, t5767, t74901, t820, t86374, t86377, t86381, t86552, t91922, t92177, t92182, t92219);
        let t92347 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1834::<F>(t14193, t22005, t22321, t46515, t46518, t48036, t6844, t74999, t75005, t75021, t75026, t75068, t820, t85638, t86468);
        let t92378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1835::<F>(t1437, t22009, t22321, t4114, t47351, t47395, t5745, t6862, t6874, t75145, t75147, t75176, t75179, t820, t86563, t91942, t92158);
    (t92064, t92070, t92229, t92248, t92267, t92317, t92347, t92378)
}
